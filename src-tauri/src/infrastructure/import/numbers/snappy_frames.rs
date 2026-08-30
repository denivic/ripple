use snap::raw::Decoder;

use super::IwaError;

/// Apple's IWA framing wraps Snappy in a *non-conformant* chunk format:
/// 1 byte chunk type (always 0 for data) + 3-byte little-endian length,
/// repeated back to back — no stream identifier, no CRC-32C, unlike the
/// real Snappy framing format. Verified byte-for-byte against a real
/// `Index/Tables/Tile.iwa` (`00 28 00 00` header = type 0, length 0x28=40).
pub fn decode_iwa_chunks(bytes: &[u8]) -> Result<Vec<u8>, IwaError> {
    let mut decoder = Decoder::new();
    let mut out = Vec::new();
    let mut pos = 0usize;
    while pos < bytes.len() {
        if pos + 4 > bytes.len() {
            return Err(IwaError::Format("truncated IWA chunk header".to_string()));
        }
        let chunk_type = bytes[pos];
        if chunk_type != 0 {
            return Err(IwaError::Format(format!(
                "unexpected IWA chunk type {chunk_type}"
            )));
        }
        let len = u32::from_le_bytes([bytes[pos + 1], bytes[pos + 2], bytes[pos + 3], 0]) as usize;
        let start = pos + 4;
        let end = start + len;
        if end > bytes.len() {
            return Err(IwaError::Format(
                "IWA chunk length exceeds buffer".to_string(),
            ));
        }
        let decompressed = decoder
            .decompress_vec(&bytes[start..end])
            .map_err(|e| IwaError::Format(format!("snappy decode failed: {e}")))?;
        out.extend_from_slice(&decompressed);
        pos = end;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snappy_literal_chunk(data: &[u8]) -> Vec<u8> {
        // A single Snappy "literal" element: uncompressed-length varint,
        // then a literal tag (len-1 in the top 6 bits, tag bits 00), then
        // the raw bytes — the same shape real Numbers output uses for
        // small payloads.
        let mut snappy = Vec::new();
        snappy.push(data.len() as u8); // varint fits in one byte for our tests
        snappy.push(((data.len() as u8 - 1) << 2) | 0b00);
        snappy.extend_from_slice(data);

        let mut framed = Vec::new();
        framed.push(0u8);
        let len = snappy.len() as u32;
        framed.extend_from_slice(&len.to_le_bytes()[..3]);
        framed.extend_from_slice(&snappy);
        framed
    }

    #[test]
    fn decodes_a_single_chunk() {
        let payload = b"hello protobuf";
        let framed = snappy_literal_chunk(payload);
        let decoded = decode_iwa_chunks(&framed).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn decodes_multiple_back_to_back_chunks() {
        let mut framed = snappy_literal_chunk(b"first");
        framed.extend(snappy_literal_chunk(b"second"));
        let decoded = decode_iwa_chunks(&framed).unwrap();
        assert_eq!(decoded, b"firstsecond");
    }

    #[test]
    fn rejects_a_nonzero_chunk_type() {
        let mut framed = snappy_literal_chunk(b"x");
        framed[0] = 1;
        assert!(decode_iwa_chunks(&framed).is_err());
    }

    #[test]
    fn rejects_truncated_input() {
        let framed = snappy_literal_chunk(b"hello");
        assert!(decode_iwa_chunks(&framed[..3]).is_err());
    }
}
