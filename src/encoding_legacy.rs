// TODO add proper error value
pub(crate) fn decode_byte(s: String) -> Result<Vec<u8>, ()> {
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

#[cfg(test)]
mod tests {
    use crate::encoding_legacy::decode_byte;

    #[test]
    fn given_valid_bytes_then_decode() {
        assert_eq!(decode_byte("00".to_string()).unwrap(), vec![0]);
        assert_eq!(decode_byte("01".to_string()).unwrap(), vec![1]);
        assert_eq!(decode_byte("ff".to_string()).unwrap(), vec![255]);
        assert_eq!(decode_byte("0001".to_string()).unwrap(), vec![0, 1]);
        assert_eq!(decode_byte("0102".to_string()).unwrap(), vec![1, 2]);
        assert_eq!(decode_byte("ff00".to_string()).unwrap(), vec![255, 0]);
        assert_eq!(decode_byte("ff01".to_string()).unwrap(), vec![255, 1]);
        assert_eq!(decode_byte("ff02".to_string()).unwrap(), vec![255, 2]);
        assert_eq!(decode_byte("ff03".to_string()).unwrap(), vec![255, 3]);
        assert_eq!(decode_byte("ff04".to_string()).unwrap(), vec![255, 4]);
        assert_eq!(decode_byte("ff05".to_string()).unwrap(), vec![255, 5]);
        assert_eq!(decode_byte("ff06".to_string()).unwrap(), vec![255, 6]);
        assert_eq!(decode_byte("ff07".to_string()).unwrap(), vec![255, 7]);
        assert_eq!(decode_byte("ff08".to_string()).unwrap(), vec![255, 8]);
        assert_eq!(decode_byte("ff09".to_string()).unwrap(), vec![255, 9]);
        assert_eq!(decode_byte("ff0a".to_string()).unwrap(), vec![255, 10]);
        assert_eq!(decode_byte("ff0b".to_string()).unwrap(), vec![255, 11]);
        assert_eq!(decode_byte("ff0c".to_string()).unwrap(), vec![255, 12]);
        assert_eq!(decode_byte("ff0d".to_string()).unwrap(), vec![255, 13]);
        assert_eq!(decode_byte("ff0e".to_string()).unwrap(), vec![255, 14]);
        assert_eq!(decode_byte("ff0f".to_string()).unwrap(), vec![255, 15]);
        assert_eq!(decode_byte("ff10".to_string()).unwrap(), vec![255, 16]);
        assert_eq!(decode_byte("ff11".to_string()).unwrap(), vec![255, 17]);
    }

    #[test]
    fn given_invalid_bytes_then_error() {
        assert!(decode_byte("0".to_string()).is_err());
        assert!(decode_byte("0z".to_string()).is_err());
        assert!(decode_byte("z0".to_string()).is_err());
        assert!(decode_byte("z".to_string()).is_err());
    }
}
