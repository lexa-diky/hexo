use std::num::ParseIntError;

pub fn decode_byte(s: String) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn to_shrunk_bytes(value: usize) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut value = value;
    while value > 0 {
        bytes.push((value & 0xFF) as u8);
        value >>= 8;
    }
    bytes
}