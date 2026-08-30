use super::IwaError;

/// A generic protobuf wire-format field, decoded without any `.proto`
/// schema — see plan-v1.md: rather than compiling Apple's several-hundred
/// message set, we walk the wire format directly and read only the field
/// numbers we know about, skipping everything else by construction (an
/// unrecognized tag is simply never looked up).
// Fixed64/Fixed32 carry no accessor and their payload is never read: they
// exist so `parse_fields` stays a *complete* wire-format walker (a message
// with a double/float field elsewhere wouldn't error out just because we
// don't happen to need that field), not because anything reads them today.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum WireValue<'a> {
    Varint(u64),
    Fixed64(u64),
    LengthDelimited(&'a [u8]),
    Fixed32(u32),
}

impl<'a> WireValue<'a> {
    pub fn as_varint(&self) -> Option<u64> {
        match self {
            WireValue::Varint(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<&'a [u8]> {
        match self {
            WireValue::LengthDelimited(b) => Some(b),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Field<'a> {
    pub number: u32,
    pub value: WireValue<'a>,
}

pub fn first<'a>(fields: &'a [Field<'a>], number: u32) -> Option<&'a WireValue<'a>> {
    fields.iter().find(|f| f.number == number).map(|f| &f.value)
}

pub fn all<'a, 'b>(
    fields: &'b [Field<'a>],
    number: u32,
) -> impl Iterator<Item = &'b WireValue<'a>> {
    fields
        .iter()
        .filter(move |f| f.number == number)
        .map(|f| &f.value)
}

/// Parses every top-level field in a message buffer. Does not recurse —
/// callers call this again on a `LengthDelimited` field's bytes to descend
/// into a submessage, which is exactly the schema-less navigation the plan
/// calls for.
pub fn parse_fields(buf: &[u8]) -> Result<Vec<Field<'_>>, IwaError> {
    let mut fields = Vec::new();
    let mut pos = 0usize;
    while pos < buf.len() {
        let (tag, tag_len) = read_varint(buf, pos)?;
        pos += tag_len;
        let field_number = (tag >> 3) as u32;
        let wire_type = (tag & 0x7) as u8;
        match wire_type {
            0 => {
                let (v, len) = read_varint(buf, pos)?;
                pos += len;
                fields.push(Field {
                    number: field_number,
                    value: WireValue::Varint(v),
                });
            }
            1 => {
                let bytes = take(buf, pos, 8)?;
                pos += 8;
                fields.push(Field {
                    number: field_number,
                    value: WireValue::Fixed64(u64::from_le_bytes(bytes.try_into().unwrap())),
                });
            }
            2 => {
                let (len, len_len) = read_varint(buf, pos)?;
                pos += len_len;
                let bytes = take(buf, pos, len as usize)?;
                pos += len as usize;
                fields.push(Field {
                    number: field_number,
                    value: WireValue::LengthDelimited(bytes),
                });
            }
            5 => {
                let bytes = take(buf, pos, 4)?;
                pos += 4;
                fields.push(Field {
                    number: field_number,
                    value: WireValue::Fixed32(u32::from_le_bytes(bytes.try_into().unwrap())),
                });
            }
            other => {
                return Err(IwaError::Format(format!(
                    "unsupported protobuf wire type {other}"
                )))
            }
        }
    }
    Ok(fields)
}

/// Unpacks a packed-repeated varint field (`[packed=true]` in the .proto —
/// used for e.g. `MessageInfo.version`, `FieldPath.path`): a length-delimited
/// byte string that is itself a back-to-back sequence of varints. Not
/// called by the navigation path yet (it doesn't need `version`/`path`),
/// but it's real, tested wire-format decoding — ready for whenever a field
/// that needs it is.
#[allow(dead_code)]
pub fn unpack_varints(buf: &[u8]) -> Result<Vec<u64>, IwaError> {
    let mut out = Vec::new();
    let mut pos = 0;
    while pos < buf.len() {
        let (v, len) = read_varint(buf, pos)?;
        out.push(v);
        pos += len;
    }
    Ok(out)
}

fn take(buf: &[u8], pos: usize, len: usize) -> Result<&[u8], IwaError> {
    buf.get(pos..pos + len)
        .ok_or_else(|| IwaError::Format("truncated protobuf field".to_string()))
}

pub(crate) fn read_varint(buf: &[u8], mut pos: usize) -> Result<(u64, usize), IwaError> {
    let start = pos;
    let mut result: u64 = 0;
    let mut shift = 0u32;
    loop {
        let byte = *buf
            .get(pos)
            .ok_or_else(|| IwaError::Format("truncated varint".to_string()))?;
        result |= ((byte & 0x7f) as u64) << shift;
        pos += 1;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift >= 64 {
            return Err(IwaError::Format("varint too long".to_string()));
        }
    }
    Ok((result, pos - start))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_single_varint_field() {
        // field 1, wire type 0, value 150 (protobuf spec's own canonical example)
        let buf = [0x08, 0x96, 0x01];
        let fields = parse_fields(&buf).unwrap();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].number, 1);
        assert_eq!(fields[0].value.as_varint(), Some(150));
    }

    #[test]
    fn parses_a_length_delimited_field() {
        // field 2, wire type 2, 3-byte payload "abc"
        let buf = [0x12, 0x03, b'a', b'b', b'c'];
        let fields = parse_fields(&buf).unwrap();
        assert_eq!(fields[0].number, 2);
        assert_eq!(fields[0].value.as_bytes(), Some(&b"abc"[..]));
    }

    #[test]
    fn unpacks_packed_varints() {
        // three single-byte varints back to back: 1, 0, 5
        let unpacked = unpack_varints(&[0x01, 0x00, 0x05]).unwrap();
        assert_eq!(unpacked, vec![1, 0, 5]);
    }

    #[test]
    fn first_and_all_find_by_field_number() {
        let buf = [0x08, 0x01, 0x08, 0x02, 0x10, 0x03];
        let fields = parse_fields(&buf).unwrap();
        assert_eq!(first(&fields, 1).and_then(|v| v.as_varint()), Some(1));
        assert_eq!(
            all(&fields, 1)
                .filter_map(|v| v.as_varint())
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
    }

    #[test]
    fn errors_on_truncated_length_delimited_field() {
        let buf = [0x12, 0x05, b'a', b'b']; // claims length 5, only 2 bytes follow
        assert!(parse_fields(&buf).is_err());
    }

    /// Byte-for-byte the ArchiveInfo from a real, minimal Numbers file's
    /// `Index/Tables/Tile.iwa` (an empty default table), hand-decoded and
    /// verified field-by-field against TSPArchiveMessages.proto before
    /// being pinned here as a regression test.
    #[test]
    fn decodes_a_real_archive_info() {
        let archive_info = [
            0x08, 0xa1, 0x9a, 0x37, // field 1 (identifier) varint = 904481
            0x12, 0x13, // field 2 (message_infos), length 19
            0x08, 0xf2, 0x2e, // MessageInfo field 1 (type) varint = 6002
            0x12, 0x03, 0x01, 0x00, 0x05, // MessageInfo field 2 (version, packed) = [1, 0, 5]
            0x18, 0x0c, // MessageInfo field 3 (length) varint = 12
            0x22, 0x07, 0x0a, 0x03, 0x0a, 0x01, 0x07, 0x18,
            0x00, // MessageInfo field 4 (field_infos)
        ];
        let fields = parse_fields(&archive_info).unwrap();
        assert_eq!(first(&fields, 1).and_then(|v| v.as_varint()), Some(904481));

        let message_info_bytes = first(&fields, 2).and_then(|v| v.as_bytes()).unwrap();
        let message_info = parse_fields(message_info_bytes).unwrap();
        assert_eq!(
            first(&message_info, 1).and_then(|v| v.as_varint()),
            Some(6002)
        );
        assert_eq!(
            first(&message_info, 3).and_then(|v| v.as_varint()),
            Some(12)
        );

        let version_bytes = first(&message_info, 2).and_then(|v| v.as_bytes()).unwrap();
        assert_eq!(unpack_varints(version_bytes).unwrap(), vec![1, 0, 5]);
    }
}
