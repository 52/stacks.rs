use crate::prelude::*;

pub(crate) const C32_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

pub(crate) const C32_BYTE_MAP: [i8; 128] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15, 16, 17, 1,
    18, 19, 1, 20, 21, 0, 22, 23, 24, 25, 26, -1, 27, 28, 29, 30, 31, -1, -1, -1, -1, -1, -1, 10,
    11, 12, 13, 14, 15, 16, 17, 1, 18, 19, 1, 20, 21, 0, 22, 23, 24, 25, 26, -1, 27, 28, 29, 30,
    31, -1, -1, -1, -1, -1,
];

pub(crate) fn c32_encode(data: &[u8]) -> Result<String> {
    let mut encoded = Vec::new();

    let mut buffer = 0u32;
    let mut bits = 0;

    for &byte in data.iter().rev() {
        buffer |= (byte as u32) << bits;
        bits += 8;

        while bits >= 5 {
            encoded.push(C32_ALPHABET[(buffer & 0x1F) as usize]);
            buffer >>= 5;
            bits -= 5;
        }
    }

    if bits > 0 {
        encoded.push(C32_ALPHABET[(buffer & 0x1F) as usize]);
    }

    while let Some(i) = encoded.pop() {
        if i != C32_ALPHABET[0] {
            encoded.push(i);
            break;
        }
    }

    for i in data {
        if *i == 0 {
            encoded.push(C32_ALPHABET[0]);
        } else {
            break;
        }
    }

    encoded.reverse();
    String::from_utf8(encoded).map_err(|_| Error::Generic)
}

pub(crate) fn c32_decode(input: impl Into<String>) -> Result<Vec<u8>> {
    let input = input.into();

    if !input.is_ascii() {
        return Err(Error::Generic);
    }

    let input = {
        let mut buffer: Vec<u8> = Vec::with_capacity(input.len());

        for i in input.as_bytes().into_iter().rev() {
            let byte = C32_BYTE_MAP.get(*i as usize).unwrap_or_else(|| &-1);

            if byte.is_negative() {
                return Err(Error::Generic);
            }

            buffer.push(*byte as u8);
        }

        buffer
    };

    let mut decoded = Vec::new();
    let mut carry = 0u16;
    let mut carry_bits = 0;

    for bits in &input {
        carry |= (*bits as u16) << carry_bits;
        carry_bits += 5;

        while carry_bits >= 8 {
            decoded.push((carry & 0xFF) as u8);
            carry >>= 8;
            carry_bits -= 8;
        }
    }

    if carry_bits > 0 {
        decoded.push(carry as u8);
    }

    while let Some(i) = decoded.pop() {
        if i != 0 {
            decoded.push(i);
            break;
        }
    }

    for i in input.iter().rev() {
        if *i == 0 {
            decoded.push(0);
        } else {
            break;
        }
    }

    decoded.reverse();
    Ok(decoded)
}

mod tests {

    #[test]
    fn test_c32_encode() {
        let input = vec![1, 2, 3, 4, 6, 1, 2, 6, 2, 3, 6, 9, 4, 0, 0];
        let encoded = super::c32_encode(&input).unwrap();
        assert_eq!(encoded, "41061060410C0G30R4G8000");
    }

    #[test]
    fn test_c32_decode() {
        let input = vec![1, 2, 3, 4, 6, 1, 2, 6, 2, 3, 6, 9, 4, 0, 0];
        let encoded = super::c32_encode(&input).unwrap();
        let decoded = super::c32_decode(encoded).unwrap();
        assert_eq!(input, decoded);
    }

    #[test]
    fn test_randomized_input() {
        use rand::{thread_rng, Rng, RngCore};
        let mut rng = thread_rng();

        for _ in 0..100 {
            let len = rng.gen_range(0..=1000);
            let mut input = vec![0u8; len];
            rng.fill_bytes(&mut input);

            let encoded = super::c32_encode(&input).unwrap();
            let decoded = super::c32_decode(encoded).unwrap();
            assert_eq!(decoded, input);
        }
    }
}
