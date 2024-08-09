use densely_packed_ternary::Engine;
use densely_packed_ternary::DPT;
use densely_packed_ternary::T0;

fn main() {
    let engine = DPT::new();
    // 243: 3 ^ 5
    for idx in 0..243 {
        let mut chunk = [T0; 5];
        let mut t = idx;
        for elem in chunk.iter_mut() {
            *elem = (t % 3).try_into().unwrap();
            t /= 3;
        }
        let mut encoded = [0; 1];
        engine.encode_slice(&mut encoded, &chunk);
        println!("{:03} {:08b} {:?}", idx, encoded[0], chunk);
    }
}
