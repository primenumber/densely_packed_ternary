use crate::engine::Engine;
use crate::trit::*;

// Base243PreDiv Encode/Decode
#[derive(Clone, Debug, Default)]
pub struct Base243PreDiv {}

const CHUNK_SIZE: usize = 5;

impl Base243PreDiv {
    pub fn new() -> Base243PreDiv {
        Base243PreDiv {}
    }

    // encoding trits into a byte (up to 5 trits)
    fn encode_chunk(&self, chunk: &[Trit]) -> u8 {
        assert!(chunk.len() <= CHUNK_SIZE);
        let chunk_filled = {
            let mut ary = [T0; CHUNK_SIZE];
            ary[0..chunk.len()].copy_from_slice(chunk);
            ary
        };
        let mut value = 0;
        let mut pow3 = 1;
        for elem in chunk_filled.iter().rev() {
            value += u8::from(*elem) * pow3;
            pow3 *= 3;
        }
        ((value as u16 * 256 + 242) / 243) as u8
    }

    fn decode_chunk(&self, mut encoded_byte: u8) -> Option<[Trit; CHUNK_SIZE]> {
        let mut result = [Default::default(); CHUNK_SIZE];
        for elem in result.iter_mut() {
            let extended = encoded_byte as u16 * 3;
            *elem = Trit::try_from((extended >> 8) as u8).ok()?;
            encoded_byte = extended as u8;
        }
        Some(result)
    }
}

impl Engine for Base243PreDiv {
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
    fn test_encode_decode_chunk() {
        let engine = Base243PreDiv::new();
        // 243: 3 ^ 5
        for idx in 0..243 {
            let mut chunk = [T0; CHUNK_SIZE];
            let mut t = idx;
            for elem in chunk.iter_mut() {
                *elem = (t % 3).try_into().unwrap();
                t /= 3;
            }
            let encoded = engine.encode_chunk(&chunk);
            let decoded = engine.decode_chunk(encoded);
            assert_eq!(Some(chunk), decoded);
        }
    }

    #[test]
    fn test_encode_decode() {
        let engine = Base243PreDiv::new();
        let original = [T0, T2, T1, T1, T0, T1, T0, T2, T2];
        assert_eq!(engine.encoded_size(&original), 2);
        let mut encoded = [0u8; 2];
        engine.encode_slice(&mut encoded, &original);

        assert!(engine.decoded_size(&encoded) >= 9);
        let mut decoded = [Default::default(); 9];
        engine.decode_slice(&mut decoded, &encoded);
        assert_eq!(original, decoded);
    }
}
