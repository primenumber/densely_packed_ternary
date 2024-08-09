use crate::engine::Engine;
use crate::trit::*;

// Binary Coded Ternary
#[derive(Clone, Debug, Default)]
pub struct BCT {}

const CHUNK_SIZE: usize = 4;

impl BCT {
    pub fn new() -> BCT {
        BCT {}
    }

    // encoding trits into a byte (up to 4 trits)
    fn encode_chunk(&self, chunk: &[Trit]) -> u8 {
        assert!(chunk.len() <= CHUNK_SIZE);
        let mut value = 0;
        for (index, elem) in chunk.iter().enumerate() {
            value |= u8::from(*elem) << (index * 2);
        }
        value
    }

    fn decode_chunk(&self, mut encoded_byte: u8) -> Option<[Trit; CHUNK_SIZE]> {
        let mut result = [Default::default(); CHUNK_SIZE];
        for elem in result.iter_mut() {
            *elem = Trit::try_from(encoded_byte & 0b11).ok()?;
            encoded_byte >>= 2;
        }
        Some(result)
    }
}

impl Engine for BCT {
    fn encoded_size(&self, trits: &[Trit]) -> usize {
        // CHUNK_SIZE trits -> 1 byte
        (trits.len() + CHUNK_SIZE - 1) / CHUNK_SIZE
    }

    fn encode_slice(&self, encoded: &mut [u8], trits: &[Trit]) -> Option<usize> {
        let size = self.encoded_size(trits);
        if size > encoded.len() {
            return None;
        }

        for (buf_byte, chunk) in encoded.iter_mut().zip(trits.chunks(CHUNK_SIZE)) {
            *buf_byte = self.encode_chunk(chunk);
        }
        Some(size)
    }

    fn decoded_size(&self, encoded: &[u8]) -> usize {
        encoded.len() * CHUNK_SIZE
    }

    fn decode_slice(&self, trits: &mut [Trit], encoded: &[u8]) -> Option<()> {
        if trits.len() > self.decoded_size(encoded) {
            return None;
        }
        for (chunk, buf_byte) in trits.chunks_mut(CHUNK_SIZE).zip(encoded) {
            chunk.copy_from_slice(&self.decode_chunk(*buf_byte)?[0..chunk.len()]);
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let engine = BCT::new();
        let original = [T0, T2, T1, T1, T0, T1, T0, T2, T2];

        assert_eq!(engine.encoded_size(&original), 3);
        let mut encoded = [0u8; 3];
        engine.encode_slice(&mut encoded, &original);

        assert!(engine.decoded_size(&encoded) >= 9);
        let mut decoded = [Default::default(); 9];
        engine.decode_slice(&mut decoded, &encoded);
        assert_eq!(original, decoded);
    }
}
