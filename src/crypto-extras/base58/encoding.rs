use crate::crypto_extras::base58::Base58Error;

pub(crate) const B58_ALPHABET: &[u8; 58] =
    b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub(crate) const B58_BYTE_MAP: [i8; 256] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, -1, -1, -1, -1, -1, -1, -1, 9, 10, 11, 12, 13, 14, 15, 16, -1,
    17, 18, 19, 20, 21, -1, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, -1, -1, -1, -1, -1, -1, 33,
    34, 35, 36, 37, 38, 39, 40, 41, 42, 43, -1, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
    57, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
];

pub(crate) fn b58_encode(data: &[u8]) -> String {
    let mut zeros = 0;
    while zeros < data.len() && data[zeros] == 0 {
        zeros += 1;
    }

    let mut result = Vec::with_capacity(data.len() * 138 / 100 + 1);
    let mut big_int = data.to_vec();

    while !big_int.is_empty() {
        let mut rem = 0;
        let mut temp = Vec::with_capacity(big_int.len());

        for b in &big_int {
            let cur = (rem << 8) + *b as u32;
            let div = cur / 58;
            rem = cur % 58;
            if !temp.is_empty() || div != 0 {
                temp.push(div as u8);
            }
        }
        result.push(B58_ALPHABET[rem as usize] as char);
        big_int = temp;
    }

    for _ in 0..zeros {
        result.push(B58_ALPHABET[0] as char);
    }

    result.reverse();
    result.into_iter().collect()
}

pub(crate) fn b58_decode(encoded: impl Into<String>) -> Result<Vec<u8>, Base58Error> {
    let encoded = encoded.into();

    if encoded.is_empty() {
        return Ok(vec![]);
    }

    let mut zeros = 0;
    while zeros < encoded.len() && encoded.as_bytes()[zeros] == b'1' {
        zeros += 1;
    }

    let mut big_int = vec![0u8; encoded.len() * 733 / 1000 + 1];

    for c in encoded.chars() {
        let index = B58_BYTE_MAP[c as usize];

        if index == -1 {
            return Err(Base58Error::InvalidChar(c));
        }

        let mut carry = index as u32;

        for byte in big_int.iter_mut().rev() {
            carry += *byte as u32 * 58;
            *byte = (carry & 0xFF) as u8;
            carry >>= 8;
        }
    }

    while !big_int.is_empty() && big_int[0] == 0 {
        big_int.remove(0);
    }

    let mut result = vec![0u8; zeros];
    result.extend_from_slice(&big_int);

    Ok(result)
}

mod tests {

    #[test]
    fn test_long_encode() {
        let input = b"ji1bAyOeHisrHIT1zCvy4RtPee3l3BQYI7AxPryRcuXQ4myW4cZlJF861RZif9lgrigWw0bKMXtMUwngfm51jNWDXBdqjYG4PFg6fOFhne7jh61VrhDhdOBSq6UTXlbYYHB4HISWsoYj0tL2S9YfHiHbAlLscNrdngkcPKQaXa1WqcdDnMlnDXsWT9JAHdudnNzyX3nRWrZCZZpw9BTHxYKB5ptcjU7T12MpDFIlELprtqtNGyPeTNgbOG1x2yz8XElLoziXrIdgZczGBSmbrFAiA0FOb4zxVi10pLVt9x3zAtakfO5KQvjCWKDnWmK2nE8DTY3fcn3QqYBxA0756mNrzPjvfikqyv5FUmwHvuDtaLtU2U3XycFoSVkohOste3rrR7sCPtCgug0AtWXCJSsuCFSafFemQwz1sCqBETy6dTUiezAb6XTA9VtMMTJetOeoNIYTGAZp9CDHWVUAtpQhylBHwfb2rzlijR6nhwYXpMQ78zYZ";
        let encoded = super::b58_encode(input);
        assert_eq!(encoded, "D3fDjDoPWxoGK1Bu8ySCct98oPWuvGDeFd6ZZpvyPCoQRMaic2sEruSiB9NguzczheNDQi6gENfdxbjYtNcCU71TcikDrpjWefhmii5ecF1WpC8QKAAGfCwbyBRWrGpBi6kB8wbkMbHBtvT8DXVkwMkjXDxMoRqCKJy3KJrMig84csPV5zmRLBMZuW6Q7b4xykCToVemtF5eTWLYGB4EzfEfQShc3yxSZxENXqn5WTga2PbXxPyBY29wvHcUcTicc43UAsvmxATpNmy34BeJUKPt6g34KKL7T86dBWZFLj96svrMFH7NHkTQXbhrSQ7SVGjKivBMJECFviLc5aC9BAXmBQch7ofxQ26exE97oQ8crLgSbX7avtiU7cuUVMd4ussRsXEv2nPnVbSECfh2f9txzbHe2cA1zC357rkjkgvXNfnifBZu1HvcSw4hxagnSa2brBx65VdKWh9dmyayXVzPtZHX6kThqXs8dZqNZTwuNtkEBcCfpDo9ZJg2LRHxfUmxcYVBfbaLj54ZkFLvwFa8YFVq9FUkGupzbNgC1bf8eSVJ6jmFt4a534MxwMyJouYKfMK7HH9dBc5vecrMgFgTsfZ65BHWnzqaVTQwY99nqVGkst4ueWVEsuNWGBG6vKmsKKBHQWVbiiDES9jgfyUWPDmgzyhckow19rE5MLm");
    }

    #[test]
    fn test_long_decode() {
        let input = b"ji1bAyOeHisrHIT1zCvy4RtPee3l3BQYI7AxPryRcuXQ4myW4cZlJF861RZif9lgrigWw0bKMXtMUwngfm51jNWDXBdqjYG4PFg6fOFhne7jh61VrhDhdOBSq6UTXlbYYHB4HISWsoYj0tL2S9YfHiHbAlLscNrdngkcPKQaXa1WqcdDnMlnDXsWT9JAHdudnNzyX3nRWrZCZZpw9BTHxYKB5ptcjU7T12MpDFIlELprtqtNGyPeTNgbOG1x2yz8XElLoziXrIdgZczGBSmbrFAiA0FOb4zxVi10pLVt9x3zAtakfO5KQvjCWKDnWmK2nE8DTY3fcn3QqYBxA0756mNrzPjvfikqyv5FUmwHvuDtaLtU2U3XycFoSVkohOste3rrR7sCPtCgug0AtWXCJSsuCFSafFemQwz1sCqBETy6dTUiezAb6XTA9VtMMTJetOeoNIYTGAZp9CDHWVUAtpQhylBHwfb2rzlijR6nhwYXpMQ78zYZ";
        let encoded = super::b58_encode(input);
        let decoded = super::b58_decode(encoded).unwrap();
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_randomized_input() {
        use rand::{thread_rng, Rng, RngCore};
        let mut rng = thread_rng();

        for _ in 0..100 {
            let len = rng.gen_range(0..=1000);
            let mut input = vec![0u8; len];
            rng.fill_bytes(&mut input);

            let encoded = super::b58_encode(&input);
            let decoded = super::b58_decode(encoded).unwrap();
            assert_eq!(decoded, input);
        }
    }

    #[test]
    fn test_invalid_char_error() {
        for c in "^&*(@#%!~`?><,.;:{]}[{|)-_=+§äöüßÄÖÜ".chars() {
            let input = format!("{}", c);

            let decoded = super::b58_decode(input);
            assert_eq!(decoded, Err(super::Base58Error::InvalidChar(c)));
        }
    }
}
