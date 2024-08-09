use crate::engine::Engine;
use crate::trit::*;

// Densely Packed Ternary
#[derive(Clone, Debug, Default)]
pub struct DPT {}

const CHUNK_SIZE: usize = 5;

enum EncodedSL {
    Small(u8),
    Large,
}

impl DPT {
    pub fn new() -> DPT {
        DPT {}
    }

    // encoding 2 trits into a nibble (4 bits)
    fn encode_2t_raw(&self, t0: Trit, t1: Trit) -> u8 {
        u8::from(t0) + u8::from(t1) * 3
    }

    fn encode_2t(&self, t0: Trit, t1: Trit) -> EncodedSL {
        let raw = self.encode_2t_raw(t0, t1);
        if raw < 8 {
            EncodedSL::Small(raw)
        } else {
            EncodedSL::Large
        }
    }

    fn encode_1t(&self, t: Trit) -> EncodedSL {
        let raw = u8::from(t);
        if raw < 2 {
            EncodedSL::Small(raw)
        } else {
            EncodedSL::Large
        }
    }

    fn decode_2t(&self, encoded: EncodedSL) -> (Trit, Trit) {
        let raw = if let EncodedSL::Small(small) = encoded {
            assert!(small < 8);
            small
        } else {
            8
        };

        let raw0 = raw % 3;
        let raw1 = raw / 3;
        (raw0.try_into().unwrap(), raw1.try_into().unwrap())
    }

    fn decode_1t(&self, encoded: EncodedSL) -> Trit {
        let raw = if let EncodedSL::Small(small) = encoded {
            assert!(small < 2);
            small
        } else {
            2
        };
        raw.try_into().unwrap()
    }

    // encoding trits into a byte (up to 5 trits)
    fn encode_chunk(&self, chunk: &[Trit]) -> u8 {
        assert!(chunk.len() <= CHUNK_SIZE);
        let chunk_filled = {
            let mut ary = [T0; CHUNK_SIZE];
            ary[0..chunk.len()].copy_from_slice(chunk);
            ary
        };
        // Encode every two values.
        let first_2t = self.encode_2t(chunk_filled[0], chunk_filled[1]);
        let second_2t = self.encode_2t(chunk_filled[2], chunk_filled[3]);
        let last_1t = self.encode_1t(chunk_filled[4]);

        if let EncodedSL::Small(first) = first_2t {
            if let EncodedSL::Small(second) = second_2t {
                // first: small, second: small
                let lower_4t = first | (second << 4);
                if let EncodedSL::Small(last) = last_1t {
                    lower_4t | (last << 3)
                } else {
                    lower_4t | 0b1000_0000
                }
            } else {
                // first:small, second: large
                let indicator = 0b1000_1100;
                indicator | (first << 4) | u8::from(chunk_filled[4])
            }
        } else if let EncodedSL::Small(second) = second_2t {
            // first: large, second: small
            let indicator = 0b1000_1000;
            indicator | (second << 4) | u8::from(chunk_filled[4])
        } else {
            // first: large, second: large
            let indicator = 0b1000_1011;
            indicator | (u8::from(chunk_filled[4]) << 4)
        }
    }

    fn decode_chunk(&self, encoded_byte: u8) -> [Trit; CHUNK_SIZE] {
        let indicator_3 = (encoded_byte >> 3) & 1;
        let indicator_7 = (encoded_byte >> 7) & 1;
        let (first_2t, second_2t, last_1t) = if indicator_3 != 0 && indicator_7 != 0 {
            let indicator_0 = encoded_byte & 1;
            let indicator_1 = (encoded_byte >> 1) & 1;
            if indicator_0 != 0 && indicator_1 != 0 {
                let indicator_5 = (encoded_byte >> 5) & 1;
                // first: large, second: large
                if indicator_5 != 0 {
                    (EncodedSL::Large, EncodedSL::Large, EncodedSL::Large)
                } else {
                    let last_small = (encoded_byte >> 4) & 1;
                    (
                        EncodedSL::Large,
                        EncodedSL::Large,
                        EncodedSL::Small(last_small),
                    )
                }
            } else {
                let indicator_2 = (encoded_byte >> 2) & 1;
                let last_1t = if indicator_1 != 0 {
                    EncodedSL::Large
                } else {
                    EncodedSL::Small(indicator_0)
                };
                let small_bits = (encoded_byte >> 4) & 0b111;
                if indicator_2 != 0 {
                    // first: small, second: large
                    (EncodedSL::Small(small_bits), EncodedSL::Large, last_1t)
                } else {
                    // first: large, second: small
                    (EncodedSL::Large, EncodedSL::Small(small_bits), last_1t)
                }
            }
        } else {
            // first: small second: small
            let first_2t = EncodedSL::Small(encoded_byte & 0b111);
            let second_2t = EncodedSL::Small((encoded_byte >> 4) & 0b111);
            let last_1t = if indicator_7 != 0 {
                EncodedSL::Large
            } else {
                EncodedSL::Small(indicator_3)
            };
            (first_2t, second_2t, last_1t)
        };
        let (t0, t1) = self.decode_2t(first_2t);
        let (t2, t3) = self.decode_2t(second_2t);
        let t4 = self.decode_1t(last_1t);
        [t0, t1, t2, t3, t4]
    }
}

impl Engine for DPT {
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
            chunk.copy_from_slice(&self.decode_chunk(*buf_byte)[0..chunk.len()]);
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_chunk() {
        let engine = DPT::new();
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
            assert_eq!(chunk, decoded);
        }
    }

    #[test]
    fn test_encode_decode() {
        let engine = DPT::new();
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
