use time::{Duration, PrimitiveDateTime};

use super::IwaError;
use crate::infrastructure::import::CellValue;

const SUPPORTED_VERSION: u8 = 5;
const DECIMAL128_BIAS: i32 = 0x1820;

/// Apple's reference epoch across Cocoa/CoreFoundation (`CFAbsoluteTime`),
/// which Numbers' date cells use too: 2001-01-01T00:00:00.
fn epoch() -> PrimitiveDateTime {
    PrimitiveDateTime::new(
        time::Date::from_calendar_date(2001, time::Month::January, 1).expect("valid calendar date"),
        time::Time::MIDNIGHT,
    )
}

/// Numbers' own decimal128 encoding — not IEEE 754 decimal128, a simpler
/// custom mantissa+exponent packing. Ported from a real, working parser's
/// `_unpack_decimal128` (see the Phase 5 commit message for provenance).
fn unpack_decimal128(buf: &[u8]) -> f64 {
    let exp = (((buf[15] as i32 & 0x7f) << 7) | (buf[14] as i32 >> 1)) - DECIMAL128_BIAS;
    let mut mantissa: i128 = (buf[14] & 1) as i128;
    for i in (0..14).rev() {
        mantissa = mantissa * 256 + buf[i] as i128;
    }
    let sign = if buf[15] & 0x80 != 0 { -1.0 } else { 1.0 };
    sign * (mantissa as f64) * 10f64.powi(exp)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellKind {
    Empty,
    Span,
    Number,
    Text,
    Formula,
    Date,
    Bool,
    Duration,
    FormulaError,
    RichText,
    Currency,
}

fn cell_kind(tag: u8) -> Option<CellKind> {
    match tag {
        0 => Some(CellKind::Empty),
        1 => Some(CellKind::Span),
        2 => Some(CellKind::Number),
        3 => Some(CellKind::Text),
        4 => Some(CellKind::Formula),
        5 => Some(CellKind::Date),
        6 => Some(CellKind::Bool),
        7 => Some(CellKind::Duration),
        8 => Some(CellKind::FormulaError),
        9 => Some(CellKind::RichText),
        10 => Some(CellKind::Currency),
        _ => None,
    }
}

#[derive(Debug)]
pub struct DecodedCell {
    pub value: CellValue,
    /// Set for Text/Formula/RichText cells — the caller resolves this
    /// against the table's string list (a separate archive), since this
    /// module deliberately has no dependency on that lookup.
    pub string_id: Option<i32>,
}

/// Decodes one cell's fixed-then-flag-driven record. Layout verified
/// against a real, working parser's source, not guessed: byte 0 = storage
/// version, byte 1 = cell type, bytes 8..12 = a little-endian flags
/// bitmask, then one field per set flag bit (ascending bit order) at the
/// widths below. An unrecognized version or cell type is the fallback
/// contract, not a panic or silently wrong data.
pub fn decode_cell(buf: &[u8]) -> Result<DecodedCell, IwaError> {
    if buf.len() < 12 {
        return Err(IwaError::Format(
            "cell storage buffer shorter than the fixed header".to_string(),
        ));
    }
    let version = buf[0];
    if version != SUPPORTED_VERSION {
        return Err(IwaError::UnsupportedRevision(format!(
            "this .numbers revision isn't supported yet (cell storage version {version}) — in Numbers, File \u{2192} Export To \u{2192} Excel, then import the .xlsx"
        )));
    }
    let kind = cell_kind(buf[1]).ok_or_else(|| {
        IwaError::UnsupportedRevision(format!(
            "this .numbers revision isn't supported yet (unrecognized cell type {})",
            buf[1]
        ))
    })?;
    let flags = u32::from_le_bytes(buf[8..12].try_into().unwrap());

    let mut offset = 12usize;
    let mut d128: Option<f64> = None;
    let mut double: Option<f64> = None;
    let mut seconds: Option<f64> = None;
    let mut string_id: Option<i32> = None;

    macro_rules! take {
        ($width:expr) => {{
            let bytes = buf
                .get(offset..offset + $width)
                .ok_or_else(|| IwaError::Format("truncated cell storage field".to_string()))?;
            offset += $width;
            bytes
        }};
    }

    if flags & 0x1 != 0 {
        d128 = Some(unpack_decimal128(take!(16)));
    }
    if flags & 0x2 != 0 {
        double = Some(f64::from_le_bytes(take!(8).try_into().unwrap()));
    }
    if flags & 0x4 != 0 {
        seconds = Some(f64::from_le_bytes(take!(8).try_into().unwrap()));
    }
    if flags & 0x8 != 0 {
        string_id = Some(i32::from_le_bytes(take!(4).try_into().unwrap()));
    }
    // Remaining flag bits (rich-text/style/formula/format ids, ...) carry
    // styling metadata this reader doesn't need for cell values, so
    // they're skipped by width without being individually named.
    for bit in [
        0x10u32, 0x20, 0x40, 0x80, 0x100, 0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000, 0x8000,
        0x10000, 0x20000, 0x40000,
    ] {
        if flags & bit != 0 {
            let _ = take!(4);
        }
    }

    let value = match kind {
        CellKind::Empty | CellKind::Span | CellKind::FormulaError => CellValue::Empty,
        CellKind::Number | CellKind::Currency => {
            d128.map(CellValue::Number).unwrap_or(CellValue::Empty)
        }
        CellKind::Text | CellKind::Formula | CellKind::RichText => {
            return Ok(DecodedCell {
                value: CellValue::Empty,
                string_id,
            });
        }
        CellKind::Date => seconds
            .map(|s| CellValue::DateTime(epoch() + Duration::seconds_f64(s)))
            .unwrap_or(CellValue::Empty),
        CellKind::Bool => CellValue::Bool(double.unwrap_or(0.0) > 0.0),
        // Duration cells store seconds; the app's own duration_minutes
        // fields are minutes, so convert at the boundary.
        CellKind::Duration => CellValue::Number(double.unwrap_or(0.0) / 60.0),
    };

    Ok(DecodedCell { value, string_id })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mirrors the real parser's `_pack_decimal128` so tests can construct
    /// valid buffers without hand-deriving byte sequences.
    fn pack_decimal128(value: f64) -> [u8; 16] {
        let mut buffer = [0u8; 16];
        let exp_base = if value != 0.0 {
            value.abs().log10().floor() as i64
        } else {
            0
        };
        let exp = exp_base + DECIMAL128_BIAS as i64 - 16;
        let mantissa = (value.abs() / 10f64.powi((exp - DECIMAL128_BIAS as i64) as i32)) as i128;
        buffer[15] |= (exp >> 7) as u8;
        buffer[14] |= ((exp & 0x7f) << 1) as u8;
        let mut m = mantissa;
        let mut i = 0;
        while m >= 1 {
            buffer[i] = (m & 0xff) as u8;
            i += 1;
            m /= 256;
        }
        if value < 0.0 {
            buffer[15] |= 0x80;
        }
        buffer
    }

    fn header(cell_type: u8, flags: u32) -> Vec<u8> {
        let mut buf = vec![5u8, cell_type, 0, 0, 0, 0, 0, 0];
        buf.extend_from_slice(&flags.to_le_bytes());
        buf
    }

    #[test]
    fn decodes_a_number_cell() {
        let mut buf = header(2, 0x1);
        buf.extend_from_slice(&pack_decimal128(42.0));
        let cell = decode_cell(&buf).unwrap();
        match cell.value {
            CellValue::Number(n) => assert!((n - 42.0).abs() < 1e-6),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn decodes_a_negative_fractional_number() {
        let mut buf = header(2, 0x1);
        buf.extend_from_slice(&pack_decimal128(-3.5));
        let cell = decode_cell(&buf).unwrap();
        match cell.value {
            CellValue::Number(n) => assert!((n - (-3.5)).abs() < 1e-6),
            other => panic!("expected Number, got {other:?}"),
        }
    }

    #[test]
    fn decodes_a_bool_cell() {
        let mut buf = header(6, 0x2);
        buf.extend_from_slice(&1.0f64.to_le_bytes());
        let cell = decode_cell(&buf).unwrap();
        assert_eq!(cell.value, CellValue::Bool(true));
    }

    #[test]
    fn decodes_a_duration_cell_as_minutes() {
        let mut buf = header(7, 0x2);
        buf.extend_from_slice(&120.0f64.to_le_bytes()); // 120 seconds
        let cell = decode_cell(&buf).unwrap();
        assert_eq!(cell.value, CellValue::Number(2.0)); // 2 minutes
    }

    #[test]
    fn decodes_a_date_cell_relative_to_the_2001_epoch() {
        let mut buf = header(5, 0x4);
        buf.extend_from_slice(&0.0f64.to_le_bytes()); // exactly the epoch
        let cell = decode_cell(&buf).unwrap();
        match cell.value {
            CellValue::DateTime(dt) => assert_eq!(dt, epoch()),
            other => panic!("expected DateTime, got {other:?}"),
        }
    }

    #[test]
    fn extracts_string_id_for_text_cells_without_resolving_it() {
        let mut buf = header(3, 0x8);
        buf.extend_from_slice(&42i32.to_le_bytes());
        let cell = decode_cell(&buf).unwrap();
        assert_eq!(cell.string_id, Some(42));
        assert_eq!(cell.value, CellValue::Empty);
    }

    #[test]
    fn skips_unneeded_flag_fields_to_reach_a_later_one() {
        // cell_style_id (0x20) then string_id (0x8) — flags are read in
        // ascending bit order regardless of this order, so string_id must
        // still land after the 4-byte style id that precedes it in the buffer.
        let mut buf = header(3, 0x8 | 0x20);
        buf.extend_from_slice(&7i32.to_le_bytes()); // string_id (bit 0x8, lower bit, read first)
        buf.extend_from_slice(&99i32.to_le_bytes()); // cell_style_id (bit 0x20, skipped)
        let cell = decode_cell(&buf).unwrap();
        assert_eq!(cell.string_id, Some(7));
    }

    #[test]
    fn rejects_an_unsupported_storage_version() {
        let mut buf = header(2, 0x1);
        buf[0] = 3;
        buf.extend_from_slice(&pack_decimal128(1.0));
        let err = decode_cell(&buf).unwrap_err();
        assert!(matches!(err, IwaError::UnsupportedRevision(_)));
    }

    #[test]
    fn rejects_an_unrecognized_cell_type() {
        let buf = header(200, 0);
        let err = decode_cell(&buf).unwrap_err();
        assert!(matches!(err, IwaError::UnsupportedRevision(_)));
    }

    #[test]
    fn empty_cell_type_needs_no_flags() {
        let buf = header(0, 0);
        let cell = decode_cell(&buf).unwrap();
        assert_eq!(cell.value, CellValue::Empty);
    }
}
