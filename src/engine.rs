use crate::trit::Trit;

pub trait Engine {
    fn encoded_size(&self, trits: &[Trit]) -> usize;
    fn encode_slice(&self, encoded_binary: &mut [u8], input_trits: &[Trit]) -> Option<usize>;
    fn decoded_size(&self, encoded_binary: &[u8]) -> usize;
    fn decode_slice(&self, output_trits: &mut [Trit], encoded_binary: &[u8]) -> Option<()>;
}
