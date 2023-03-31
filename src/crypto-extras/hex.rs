use std::fmt::Write;

pub(crate) fn hex_to_bytes(value: impl Into<String>) -> Option<Vec<u8>> {
    let value = value.into();

    let mut bytes = Vec::with_capacity(value.len() / 2);
    let mut chars = value.chars().peekable();

    if value.len() % 2 != 0 {
        chars.next();
    }

    while let Some(high_char) = chars.next() {
        let high = match high_char.to_digit(16) {
            Some(n) => n as u8,
            None => return None,
        };
        let low = match chars.next() {
            Some(low_char) => match low_char.to_digit(16) {
                Some(n) => n as u8,
                None => return None,
            },
            None => return None,
        };
        bytes.push((high << 4) | low);
    }

    Some(bytes)
}

pub(crate) fn bytes_to_hex(value: &[u8]) -> String {
    let mut buff = String::with_capacity(value.len() * 2);
    for b in value.iter() {
        write!(buff, "{:02x}", b).unwrap();
    }
    buff
}

mod tests {

    #[test]
    fn test_hex_conversion() {
        use crate::crypto_extras::hex::bytes_to_hex;
        use crate::crypto_extras::hex::hex_to_bytes;

        let input = "2a6b3badb7816e12cb12e3b50e6ea0d5";
        let bytes = hex_to_bytes(input).unwrap();
        let hex = bytes_to_hex(&bytes);

        assert_eq!(hex, input);
    }

    #[test]
    fn test_randomized_input() {
        use crate::crypto_extras::hex::bytes_to_hex;
        use crate::crypto_extras::hex::hex_to_bytes;
        use rand::{thread_rng, Rng, RngCore};

        let mut rng = thread_rng();

        for _ in 0..10_000 {
            let len = rng.gen_range(0..=1000);
            let mut input = vec![0u8; len];
            rng.fill_bytes(&mut input);

            let encoded = bytes_to_hex(&input);
            let decoded = hex_to_bytes(encoded).unwrap();
            assert_eq!(decoded, input);
        }
    }
}
