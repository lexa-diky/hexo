pub(crate) fn decode_bytes_from_string(s: &String) -> Result<Vec<u8>, ()> {
    (0..s.len())
        .step_by(2)
        .map(|i| {
            if s.len() < 2 {
                return Err(());
            }
            u8::from_str_radix(&s[i..i + 2], 16).map_err(|_| ())
        })
        .collect()
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