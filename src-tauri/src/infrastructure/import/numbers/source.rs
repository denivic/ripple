use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use super::archive::ArchiveMessage;
use super::bundle::Bundle;
use super::cell_storage::{decode_cell, DecodedCell};
use super::protobuf::{all, first, parse_fields, Field};
use super::registry;
use super::IwaError;
use crate::infrastructure::import::{CellValue, ImportError, Sheet, TabularSource};

pub struct NumbersSource {
    path: PathBuf,
}

impl NumbersSource {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl TabularSource for NumbersSource {
    fn sheets(&self) -> Result<Vec<Sheet>, ImportError> {
        read_sheets(&self.path).map_err(|e| match e {
            IwaError::UnsupportedRevision(msg) => ImportError::Unsupported(msg),
            other => ImportError::Format(other.to_string()),
        })
    }
}

const FALLBACK_MESSAGE: &str =
    "this .numbers revision isn't supported yet — in Numbers, File \u{2192} Export To \u{2192} Excel, then import the .xlsx";

fn read_sheets(path: &Path) -> Result<Vec<Sheet>, IwaError> {
    let bundle = Bundle::open(path)?;
    let index = bundle.index();

    // Scan directly for TableModelArchive rather than walking
    // Sheet -> TableInfoArchive -> TableModelArchive by reference: every
    // table we need to recover a grid from has one, and type-scanning is
    // far more robust to iWork-version differences in the intermediate
    // reference structure than assuming a specific reference shape.
    let mut sheets = Vec::new();
    for archives in index.values() {
        for archive in archives {
            if archive.object_type == registry::TABLE_MODEL_ARCHIVE {
                if let Some(sheet) = read_table(archive.payload, &index)? {
                    sheets.push(sheet);
                }
            }
        }
    }

    if sheets.is_empty() {
        return Err(IwaError::UnsupportedRevision(FALLBACK_MESSAGE.to_string()));
    }
    Ok(sheets)
}

fn reference_identifier(fields: &[Field], field_number: u32) -> Option<u64> {
    let bytes = first(fields, field_number)?.as_bytes()?;
    let ref_fields = parse_fields(bytes).ok()?;
    first(&ref_fields, 1)?.as_varint()
}

fn read_table<'a>(
    payload: &'a [u8],
    index: &HashMap<u64, Vec<ArchiveMessage<'a>>>,
) -> Result<Option<Sheet>, IwaError> {
    let fields = parse_fields(payload)?;
    let table_name = first(&fields, 8)
        .and_then(|v| v.as_bytes())
        .map(|b| String::from_utf8_lossy(b).into_owned())
        .unwrap_or_else(|| "Sheet1".to_string());

    let Some(data_store_bytes) = first(&fields, 4).and_then(|v| v.as_bytes()) else {
        return Ok(None);
    };
    let data_store_fields = parse_fields(data_store_bytes)?;

    let string_table = read_string_table(&data_store_fields, index);

    let Some(tiles_ref_id) = reference_identifier(&data_store_fields, 3) else {
        return Ok(None);
    };
    let Some(tile_storage_archive) = index.get(&tiles_ref_id).and_then(|v| v.first()) else {
        return Ok(None);
    };
    let tile_storage_fields = parse_fields(tile_storage_archive.payload)?;

    let mut row_map: BTreeMap<u32, Vec<CellValue>> = BTreeMap::new();
    let mut max_col = 0usize;

    for tile_ref_val in all(&tile_storage_fields, 1) {
        let Some(tile_entry_bytes) = tile_ref_val.as_bytes() else {
            continue;
        };
        let tile_entry_fields = parse_fields(tile_entry_bytes)?;
        let Some(tile_id) = reference_identifier(&tile_entry_fields, 2) else {
            continue;
        };
        let Some(tile_archive) = index
            .get(&tile_id)
            .and_then(|archives| archives.iter().find(|a| a.object_type == registry::TILE))
        else {
            continue;
        };
        let tile_fields = parse_fields(tile_archive.payload)?;

        for row_info_val in all(&tile_fields, 5) {
            let Some(row_bytes) = row_info_val.as_bytes() else {
                continue;
            };
            let row_fields = parse_fields(row_bytes)?;
            let row_index = first(&row_fields, 1)
                .and_then(|v| v.as_varint())
                .unwrap_or(0) as u32;
            let storage = first(&row_fields, 3)
                .and_then(|v| v.as_bytes())
                .unwrap_or(&[]);
            let offsets = first(&row_fields, 4)
                .and_then(|v| v.as_bytes())
                .unwrap_or(&[]);

            let mut row_cells: Vec<(usize, CellValue)> = Vec::new();
            for (col, chunk) in offsets.as_chunks::<2>().0.iter().enumerate() {
                let raw = u16::from_le_bytes(*chunk);
                if raw == 0xffff {
                    continue;
                }
                let start = raw as usize;
                let Some(cell_bytes) = storage.get(start..) else {
                    continue;
                };
                // A single unrecognized cell doesn't fail the whole import
                // — the fallback contract applies per-cell, not per-file.
                if let Ok(decoded) = decode_cell(cell_bytes) {
                    row_cells.push((col, resolve_cell_value(decoded, &string_table)));
                    max_col = max_col.max(col + 1);
                }
            }
            if !row_cells.is_empty() {
                let width = row_cells.iter().map(|(c, _)| c + 1).max().unwrap_or(0);
                let mut row = vec![CellValue::Empty; width];
                for (c, v) in row_cells {
                    row[c] = v;
                }
                row_map.insert(row_index, row);
            }
        }
    }

    if row_map.is_empty() {
        return Ok(None);
    }

    let max_row = *row_map.keys().max().unwrap();
    let mut rows = Vec::with_capacity(max_row as usize + 1);
    for r in 0..=max_row {
        let mut row = row_map.remove(&r).unwrap_or_default();
        row.resize(max_col, CellValue::Empty);
        rows.push(row);
    }

    Ok(Some(Sheet {
        name: table_name,
        rows,
    }))
}

/// Best-effort: collects every length-delimited field in the
/// `TableDataList` payload that decodes as UTF-8, in wire order, and uses
/// that position as the string id. `TableDataList`'s exact schema isn't in
/// any public source this reader could be checked against real string
/// content — see the Phase 5 commit message. Getting this wrong loses only
/// text cells; numbers, dates and booleans don't depend on it.
fn read_string_table<'a>(
    data_store_fields: &[Field<'a>],
    index: &HashMap<u64, Vec<ArchiveMessage<'a>>>,
) -> Vec<String> {
    let Some(id) = reference_identifier(data_store_fields, 4) else {
        return Vec::new();
    };
    let Some(archive) = index.get(&id).and_then(|archives| {
        archives
            .iter()
            .find(|a| a.object_type == registry::TABLE_DATA_LIST)
    }) else {
        return Vec::new();
    };
    let Ok(fields) = parse_fields(archive.payload) else {
        return Vec::new();
    };
    fields
        .iter()
        .filter_map(|f| f.value.as_bytes())
        .filter_map(|b| std::str::from_utf8(b).ok())
        .map(|s| s.to_string())
        .collect()
}

fn resolve_cell_value(decoded: DecodedCell, string_table: &[String]) -> CellValue {
    match decoded.string_id {
        Some(id) if id >= 0 => string_table
            .get(id as usize)
            .cloned()
            .map(CellValue::Text)
            .unwrap_or(CellValue::Text(String::new())),
        Some(_) => CellValue::Text(String::new()),
        None => decoded.value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal protobuf encoder for building synthetic IWA fixtures by
    /// hand — the plan's own risk-mitigation strategy for a reader that
    /// can't be checked against a real `.numbers` file in this environment.
    struct Encoder {
        buf: Vec<u8>,
    }

    impl Encoder {
        fn new() -> Self {
            Self { buf: Vec::new() }
        }
        fn varint(&mut self, mut v: u64) {
            loop {
                let byte = (v & 0x7f) as u8;
                v >>= 7;
                if v == 0 {
                    self.buf.push(byte);
                    break;
                }
                self.buf.push(byte | 0x80);
            }
        }
        fn tag(&mut self, field: u32, wire_type: u8) {
            self.varint(((field as u64) << 3) | wire_type as u64);
        }
        fn varint_field(&mut self, field: u32, v: u64) {
            self.tag(field, 0);
            self.varint(v);
        }
        fn bytes_field(&mut self, field: u32, bytes: &[u8]) {
            self.tag(field, 2);
            self.varint(bytes.len() as u64);
            self.buf.extend_from_slice(bytes);
        }
        fn message_field(&mut self, field: u32, inner: &Encoder) {
            self.bytes_field(field, &inner.buf);
        }
        fn into_bytes(self) -> Vec<u8> {
            self.buf
        }
    }

    fn reference(identifier: u64) -> Encoder {
        let mut e = Encoder::new();
        e.varint_field(1, identifier);
        e
    }

    fn archive_info(identifier: u64, object_type: u32, payload_len: usize) -> Vec<u8> {
        let mut message_info = Encoder::new();
        message_info.varint_field(1, object_type as u64);
        message_info.varint_field(3, payload_len as u64);

        let mut archive_info = Encoder::new();
        archive_info.varint_field(1, identifier);
        archive_info.message_field(2, &message_info);

        let bytes = archive_info.into_bytes();
        let mut framed = Vec::new();
        let mut len_enc = Encoder::new();
        len_enc.varint(bytes.len() as u64);
        framed.extend_from_slice(&len_enc.into_bytes());
        framed.extend_from_slice(&bytes);
        framed
    }

    fn archive(identifier: u64, object_type: u32, payload: &[u8]) -> Vec<u8> {
        let mut out = archive_info(identifier, object_type, payload.len());
        out.extend_from_slice(payload);
        out
    }

    /// One number cell (value 7.0) in row 0, column 0 of a single-tile,
    /// single-table synthetic bundle: TableModelArchive -> DataStore ->
    /// TileStorage -> Tile -> TileRowInfo -> cellStorageBuffer. Every
    /// field number matches TSPArchiveMessages.proto / TSTArchives.proto
    /// (identifiers 1/2/3/4/5/6000/6001/6002 verified against real files —
    /// see registry.rs).
    fn build_synthetic_table() -> (u64, HashMap<u64, Vec<u8>>) {
        let mut objects: HashMap<u64, Vec<u8>> = HashMap::new();

        // Cell storage: version 5, numberCellType(2), flags=0x1 (d128), value 7.0.
        let mut cell = vec![5u8, 2, 0, 0, 0, 0, 0, 0];
        cell.extend_from_slice(&1u32.to_le_bytes()); // flags = 0x1
        cell.extend_from_slice(&pack_decimal128(7.0));

        let mut offsets = vec![0u8, 0u8]; // column 0 -> offset 0
        offsets.extend(std::iter::repeat_n(0xffu8, 2 * 254)); // columns 1..255 empty

        let mut row_info = Encoder::new();
        row_info.varint_field(1, 0); // tileRowIndex
        row_info.varint_field(2, 1); // cellCount
        row_info.bytes_field(3, &cell); // cellStorageBuffer
        row_info.bytes_field(4, &offsets); // cellOffsets

        let mut tile = Encoder::new();
        tile.varint_field(1, 1); // maxColumn
        tile.varint_field(2, 1); // maxRow
        tile.varint_field(3, 1); // numCells
        tile.varint_field(4, 1); // numrows
        tile.message_field(5, &row_info); // rowInfos
        objects.insert(200, archive(200, registry::TILE, &tile.into_bytes()));

        let mut tile_storage_tile_entry = Encoder::new();
        tile_storage_tile_entry.varint_field(1, 0); // tileid
        tile_storage_tile_entry.message_field(2, &reference(200)); // tile: Reference

        let mut tile_storage = Encoder::new();
        tile_storage.message_field(1, &tile_storage_tile_entry);
        objects.insert(300, archive(300, 9999, &tile_storage.into_bytes())); // TileStorage has no dedicated verified type id; any is fine, we reach it by reference.

        let mut data_store = Encoder::new();
        data_store.message_field(3, &reference(300)); // tiles: Reference to TileStorage

        let mut table_model = Encoder::new();
        table_model.bytes_field(8, b"Habits");
        table_model.message_field(4, &data_store);
        objects.insert(
            400,
            archive(
                400,
                registry::TABLE_MODEL_ARCHIVE,
                &table_model.into_bytes(),
            ),
        );

        (400, objects)
    }

    fn pack_decimal128(value: f64) -> [u8; 16] {
        const BIAS: i64 = 0x1820;
        let mut buffer = [0u8; 16];
        let exp_base = if value != 0.0 {
            value.abs().log10().floor() as i64
        } else {
            0
        };
        let exp = exp_base + BIAS - 16;
        let mantissa = (value.abs() / 10f64.powi((exp - BIAS) as i32)) as i128;
        buffer[15] |= (exp >> 7) as u8;
        buffer[14] |= ((exp & 0x7f) << 1) as u8;
        let mut m = mantissa;
        let mut i = 0;
        while m >= 1 {
            buffer[i] = (m & 0xff) as u8;
            i += 1;
            m /= 256;
        }
        buffer
    }

    #[test]
    fn reads_a_single_number_cell_through_the_full_navigation_chain() {
        let (table_model_id, objects) = build_synthetic_table();

        let mut index: HashMap<u64, Vec<ArchiveMessage>> = HashMap::new();
        for (id, bytes) in &objects {
            let archives = super::super::archive::parse_archives(bytes).unwrap();
            index.entry(*id).or_default().extend(archives);
        }

        let table_model_bytes = index.get(&table_model_id).unwrap()[0].payload;
        let sheet = read_table(table_model_bytes, &index).unwrap().unwrap();

        assert_eq!(sheet.name, "Habits");
        assert_eq!(sheet.rows.len(), 1);
        match sheet.rows[0][0] {
            CellValue::Number(n) => assert!((n - 7.0).abs() < 1e-6),
            ref other => panic!("expected Number, got {other:?}"),
        }
    }
}
