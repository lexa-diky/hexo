use std::num::ParseIntError;

pub(crate) fn decode_byte(s: String) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub(crate) fn parse_number(value: String, radix: u32) -> u32 {
    // implement conversion of value from string to u32 in radix
    u32::from_str_radix(&value, radix).unwrap()
}

pub(crate) fn to_shrunk_bytes(value: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut value = value;
    while value > 0 {
        bytes.push((value & 0xFF) as u8);
        value >>= 8;
    }
    bytes
}
