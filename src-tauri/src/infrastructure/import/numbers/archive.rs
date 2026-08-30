use super::protobuf::{all, first, parse_fields, read_varint};
use super::IwaError;

/// One archived object's payload, with the object identifier and type
/// (`IWAObjectType`) MessageInfo carries alongside it.
pub struct ArchiveMessage<'a> {
    pub identifier: u64,
    pub object_type: u32,
    pub payload: &'a [u8],
}

/// Walks the decompressed IWA byte stream: repeating
/// `[varint ArchiveInfo length][ArchiveInfo][payload bytes per message_info]`
/// blocks, per TSPArchiveMessages.proto (`ArchiveInfo.identifier` = field 1,
/// `.message_infos` = field 2; `MessageInfo.type` = field 1, `.length` =
/// field 3). Verified byte-for-byte against a real minimal `Tile.iwa`.
pub fn parse_archives(buf: &[u8]) -> Result<Vec<ArchiveMessage<'_>>, IwaError> {
    let mut out = Vec::new();
    let mut pos = 0usize;
    while pos < buf.len() {
        let (archive_info_len, len_len) = read_varint(buf, pos)?;
        pos += len_len;
        let archive_info_bytes = buf
            .get(pos..pos + archive_info_len as usize)
            .ok_or_else(|| IwaError::Format("truncated ArchiveInfo".to_string()))?;
        pos += archive_info_len as usize;

        let archive_info_fields = parse_fields(archive_info_bytes)?;
        let identifier = first(&archive_info_fields, 1)
            .and_then(|v| v.as_varint())
            .unwrap_or(0);

        for message_info_value in all(&archive_info_fields, 2) {
            let message_info_bytes = message_info_value.as_bytes().ok_or_else(|| {
                IwaError::Format("ArchiveInfo.message_infos was not length-delimited".to_string())
            })?;
            let message_info_fields = parse_fields(message_info_bytes)?;
            let object_type = first(&message_info_fields, 1)
                .and_then(|v| v.as_varint())
                .ok_or_else(|| IwaError::Format("MessageInfo missing type".to_string()))?
                as u32;
            let length = first(&message_info_fields, 3)
                .and_then(|v| v.as_varint())
                .ok_or_else(|| IwaError::Format("MessageInfo missing length".to_string()))?
                as usize;

            let payload = buf
                .get(pos..pos + length)
                .ok_or_else(|| IwaError::Format("truncated archive payload".to_string()))?;
            pos += length;

            out.push(ArchiveMessage {
                identifier,
                object_type,
                payload,
            });
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The full decompressed byte stream from a real minimal `Tile.iwa`:
    /// `[varint 25][ArchiveInfo, 25 bytes][12-byte Tile payload]`.
    #[test]
    fn parses_a_real_archive_stream() {
        let stream: [u8; 1 + 25 + 12] = [
            0x19, // ArchiveInfo length = 25
            0x08, 0xa1, 0x9a, 0x37, // identifier = 904481
            0x12, 0x13, // message_infos, length 19
            0x08, 0xf2, 0x2e, // type = 6002
            0x12, 0x03, 0x01, 0x00, 0x05, // version = [1,0,5]
            0x18, 0x0c, // length = 12
            0x22, 0x07, 0x0a, 0x03, 0x0a, 0x01, 0x07, 0x18, 0x00, // field_infos
            // 12-byte Tile payload: maxColumn=0, maxRow=0, numCells=0, numrows=0, storage_version=5, field7=1
            0x08, 0x00, 0x10, 0x00, 0x18, 0x00, 0x20, 0x00, 0x30, 0x05, 0x38, 0x01,
        ];
        let archives = parse_archives(&stream).unwrap();
        assert_eq!(archives.len(), 1);
        assert_eq!(archives[0].identifier, 904481);
        assert_eq!(archives[0].object_type, 6002);
        assert_eq!(archives[0].payload.len(), 12);
    }

    #[test]
    fn parses_consecutive_archives() {
        let mut stream = Vec::new();
        stream.extend_from_slice(&real_archive_bytes());
        stream.extend_from_slice(&real_archive_bytes());
        let archives = parse_archives(&stream).unwrap();
        assert_eq!(archives.len(), 2);
    }

    fn real_archive_bytes() -> Vec<u8> {
        vec![
            0x19, 0x08, 0xa1, 0x9a, 0x37, 0x12, 0x13, 0x08, 0xf2, 0x2e, 0x12, 0x03, 0x01, 0x00,
            0x05, 0x18, 0x0c, 0x22, 0x07, 0x0a, 0x03, 0x0a, 0x01, 0x07, 0x18, 0x00, 0x08, 0x00,
            0x10, 0x00, 0x18, 0x00, 0x20, 0x00, 0x30, 0x05, 0x38, 0x01,
        ]
    }
}
