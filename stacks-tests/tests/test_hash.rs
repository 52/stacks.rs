// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use std::str::FromStr;

use format as f;
use serde::json;
use serde_json as serde;
use stacks_primitives::hash::H160;
use stacks_primitives::hash::H256;
use stacks_primitives::hash::H512;
use stacks_primitives::hex::FromHex;

mod h160 {
    use super::*;

    #[test]
    fn new() {
        let bytes = H160::new([0u8; 20]);
        assert_eq!(bytes, [0u8; 20]);
    }

    #[test]
    fn zero() {
        let bytes = H160::zero();
        assert_eq!(bytes, [0u8; 20]);
    }

    #[test]
    fn repeat_byte() {
        let bytes = H160::repeat_byte(0xFF);
        assert_eq!(bytes, [0xFF; 20]);
    }

    #[test]
    fn len() {
        let bytes = H160::new([0u8; 20]);
        assert_eq!(bytes.len(), 20);
    }

    #[test]
    fn is_empty() {
        let bytes = H160::new([0u8; 20]);
        assert_eq!(bytes.is_empty(), false);
    }

    #[test]
    fn raw() {
        let bytes = H160::new([0u8; 20]);
        assert_eq!(bytes.raw(), [0u8; 20]);
    }

    #[test]
    fn from_slice() {
        let bytes = H160::from_slice(&[0u8; 20]);
        assert_eq!(bytes, [0u8; 20])
    }

    #[test]
    fn append() {
        let bytes = H160::new([0u8; 20]);
        let append = bytes.append(0);
        assert_eq!(append.len(), 21);
        assert_eq!(append.raw(), [0u8; 21]);
    }

    #[test]
    fn prepend() {
        let bytes = H160::new([0u8; 20]);
        let prepend = bytes.prepend(0);
        assert_eq!(prepend.len(), 21);
        assert_eq!(prepend.raw(), [0u8; 21]);
    }

    #[test]
    fn split() {
        let bytes = H160::new([0u8; 20]);
        let (lhs, rhs) = bytes.split::<8, 12>();
        assert_eq!(lhs.len(), 8);
        assert_eq!(rhs.len(), 12);
        assert_eq!(lhs.raw(), [0u8; 8]);
        assert_eq!(rhs.raw(), [0u8; 12]);
    }

    #[test]
    fn concat() {
        let lhs = H160::new([0u8; 20]);
        let rhs = H160::new([0u8; 20]);
        let concatenated = lhs.concat(rhs);
        assert_eq!(concatenated.len(), 40);
        assert_eq!(concatenated.raw(), [0u8; 40]);
    }

    #[test]
    #[should_panic(expected = "bad conversion: expected 20 bytes, got 3 bytes")]
    fn from_slice_panic() {
        let slice: &[u8] = &[1, 2, 3];
        let _ = H160::from_slice(slice);
    }

    #[test]
    fn random() {
        let bytes = H160::random();
        assert_ne!(bytes, H160::zero());
        assert_eq!(bytes.len(), 20);
    }

    #[test]
    fn random_with() {
        let mut rng = rand_core::OsRng;
        let bytes = H160::random_with(&mut rng);
        assert_ne!(bytes, H160::zero());
        assert_eq!(bytes.len(), 20);
    }

    #[test]
    fn iter() {
        let bytes = H160::new([0u8; 20]);
        let copy: Vec<u8> = bytes.iter().map(|i| i + 1).collect();
        assert_eq!(copy, [1u8; 20]);
    }

    #[test]
    fn iter_mut() {
        let mut bytes = H160::new([0u8; 20]);
        bytes.iter_mut().for_each(|i| *i = 1);
        assert_eq!(bytes, [1u8; 20]);
    }

    #[test]
    fn fmt_display() {
        let bytes = H160::new([0xFF; 20]);
        let fmt = "0xffffffffffffffffffffffffffffffffffffffff";
        assert_eq!(f!("{bytes}"), fmt);
    }

    #[test]
    fn fmt_debug() {
        let bytes = H160::new([0xFF; 20]);
        let fmt = "H160(0xffffffffffffffffffffffffffffffffffffffff)";
        assert_eq!(f!("{bytes:?}"), fmt);
    }

    #[test]
    fn fmt_hex_lowercase() {
        let bytes = H160::new([0xFF; 20]);
        let fmt = "0xffffffffffffffffffffffffffffffffffffffff";
        assert_eq!(f!("{bytes:x}"), fmt);
    }

    #[test]
    fn fmt_hex_uppercase() {
        let bytes = H160::new([0xFF; 20]);
        let fmt = "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        assert_eq!(f!("{bytes:X}"), fmt);
    }

    #[test]
    fn from_str() {
        let str = "0xffffffffffffffffffffffffffffffffffffffff";
        let bytes = H160::from_str(str).unwrap();
        assert_eq!(f!("{bytes}"), str);
    }

    #[test]
    fn from_hex_lowercase() {
        let str = "0xffffffffffffffffffffffffffffffffffffffff";
        let bytes = H160::from_hex(str).unwrap();
        assert_eq!(f!("{bytes:x}"), str);
    }

    #[test]
    fn from_hex_uppercase() {
        let str = "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        let bytes = H160::from_hex(str).unwrap();
        assert_eq!(f!("{bytes:X}"), str);
    }

    #[test]
    fn serde_json() {
        let bytes = H160::new([0xFF; 20]);
        let json = serde::to_string(&bytes).unwrap();
        assert_eq!(json, "\"0xffffffffffffffffffffffffffffffffffffffff\"");
        let de: H160 = serde::from_str(&json).unwrap();
        assert_eq!(de, bytes);
    }

    #[test]
    fn serde_value() {
        let bytes = H160::new([0xFF; 20]);
        let ser = serde::to_value(&bytes).unwrap();
        assert_eq!(ser, json! { "0xffffffffffffffffffffffffffffffffffffffff" });
        let de: H160 = serde::from_value(ser).unwrap();
        assert_eq!(de, bytes);
    }
}

mod h256 {
    use super::*;

    #[test]
    fn new() {
        let bytes = H256::new([0u8; 32]);
        assert_eq!(bytes, [0u8; 32]);
    }

    #[test]
    fn zero() {
        let bytes = H256::zero();
        assert_eq!(bytes, [0u8; 32]);
    }

    #[test]
    fn repeat_byte() {
        let bytes = H256::repeat_byte(0xFF);
        assert_eq!(bytes, [0xFF; 32]);
    }

    #[test]
    fn len() {
        let bytes = H256::new([0u8; 32]);
        assert_eq!(bytes.len(), 32);
    }

    #[test]
    fn is_empty() {
        let bytes = H256::new([0u8; 32]);
        assert_eq!(bytes.is_empty(), false);
    }

    #[test]
    fn raw() {
        let bytes = H256::new([0u8; 32]);
        assert_eq!(bytes.raw(), [0u8; 32]);
    }

    #[test]
    fn append() {
        let bytes = H256::new([0u8; 32]);
        let append = bytes.append(0);
        assert_eq!(append.len(), 33);
        assert_eq!(append.raw(), [0u8; 33]);
    }

    #[test]
    fn prepend() {
        let bytes = H256::new([0u8; 32]);
        let prepend = bytes.prepend(0);
        assert_eq!(prepend.len(), 33);
        assert_eq!(prepend.raw(), [0u8; 33]);
    }

    #[test]
    fn split() {
        let bytes = H256::new([0u8; 32]);
        let (lhs, rhs) = bytes.split::<2, 30>();
        assert_eq!(lhs.len(), 2);
        assert_eq!(rhs.len(), 30);
        assert_eq!(lhs.raw(), [0u8; 2]);
        assert_eq!(rhs.raw(), [0u8; 30]);
    }

    #[test]
    fn concat() {
        let lhs = H256::new([0u8; 32]);
        let rhs = H256::new([0u8; 32]);
        let concatenated = lhs.concat(rhs);
        assert_eq!(concatenated.len(), 64);
        assert_eq!(concatenated.raw(), [0u8; 64]);
    }

    #[test]
    fn from_slice() {
        let bytes = H256::from_slice(&[0u8; 32]);
        assert_eq!(bytes, [0u8; 32]);
    }

    #[test]
    #[should_panic(expected = "bad conversion: expected 32 bytes, got 3 bytes")]
    fn from_slice_panic() {
        let slice: &[u8] = &[1, 2, 3];
        let _ = H256::from_slice(slice);
    }

    #[test]
    fn random() {
        let bytes = H256::random();
        assert_ne!(bytes, H256::zero());
        assert_eq!(bytes.len(), 32);
    }

    #[test]
    fn random_with() {
        let mut rng = rand_core::OsRng;
        let bytes = H256::random_with(&mut rng);
        assert_ne!(bytes, H256::zero());
        assert_eq!(bytes.len(), 32);
    }

    #[test]
    fn iter() {
        let bytes = H256::new([0u8; 32]);
        let copy: Vec<u8> = bytes.iter().map(|i| i + 1).collect();
        assert_eq!(copy, [1u8; 32]);
    }

    #[test]
    fn iter_mut() {
        let mut bytes = H256::new([0u8; 32]);
        bytes.iter_mut().for_each(|i| *i = 1);
        assert_eq!(bytes, [1u8; 32]);
    }

    #[test]
    fn fmt_display() {
        let bytes = H256::new([0xFF; 32]);
        let fmt = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
        assert_eq!(f!("{bytes}"), fmt);
    }

    #[test]
    fn fmt_debug() {
        let bytes = H256::new([0xFF; 32]);
        let fmt = "H256(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff)";
        assert_eq!(f!("{bytes:?}"), fmt);
    }

    #[test]
    fn fmt_hex_lowercase() {
        let bytes = H256::new([0xFF; 32]);
        let fmt = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
        assert_eq!(f!("{bytes:x}"), fmt);
    }

    #[test]
    fn fmt_hex_uppercase() {
        let bytes = H256::new([0xFF; 32]);
        let fmt = "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        assert_eq!(f!("{bytes:X}"), fmt);
    }

    #[test]
    fn from_str() {
        let str = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
        let bytes = H256::from_str(str).unwrap();
        assert_eq!(f!("{bytes}"), str);
    }

    #[test]
    fn from_hex_lowercase() {
        let str = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
        let bytes = H256::from_hex(str).unwrap();
        assert_eq!(f!("{bytes:x}"), str);
    }

    #[test]
    fn from_hex_uppercase() {
        let str = "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        let bytes = H256::from_hex(str).unwrap();
        assert_eq!(f!("{bytes:X}"), str);
    }

    #[test]
    fn serde_json() {
        let bytes = H256::new([0xFF; 32]);
        let json = serde::to_string(&bytes).unwrap();
        assert_eq!(json, "\"0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"");
        let de: H256 = serde::from_str(&json).unwrap();
        assert_eq!(de, bytes);
    }

    #[test]
    #[rustfmt::skip]
    fn serde_value() {
        let bytes = H256::new([0xFF; 32]);
        let ser = serde::to_value(&bytes).unwrap();
        assert_eq!(ser, json! { "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff" });
        let de: H256 = serde::from_value(ser).unwrap();
        assert_eq!(de, bytes);
    }
}

mod h512 {
    use super::*;

    #[test]
    fn new() {
        let bytes = H512::new([0u8; 64]);
        assert_eq!(bytes, [0u8; 64]);
    }

    #[test]
    fn zero() {
        let bytes = H512::zero();
        assert_eq!(bytes, [0u8; 64]);
    }

    #[test]
    fn repeat_byte() {
        let bytes = H512::repeat_byte(0xFF);
        assert_eq!(bytes, [0xFF; 64]);
    }

    #[test]
    fn len() {
        let bytes = H512::new([0u8; 64]);
        assert_eq!(bytes.len(), 64);
    }

    #[test]
    fn is_empty() {
        let bytes = H512::new([0u8; 64]);
        assert_eq!(bytes.is_empty(), false);
    }

    #[test]
    fn raw() {
        let bytes = H512::new([0u8; 64]);
        assert_eq!(bytes.raw(), [0u8; 64]);
    }

    #[test]
    fn append() {
        let bytes = H512::new([0u8; 64]);
        let append = bytes.append(0);
        assert_eq!(append.len(), 65);
        assert_eq!(append.raw(), [0u8; 65]);
    }

    #[test]
    fn prepend() {
        let bytes = H512::new([0u8; 64]);
        let prepend = bytes.prepend(0);
        assert_eq!(prepend.len(), 65);
        assert_eq!(prepend.raw(), [0u8; 65]);
    }

    #[test]
    fn split() {
        let bytes = H512::new([0u8; 64]);
        let (lhs, rhs) = bytes.split::<32, 32>();
        assert_eq!(lhs.len(), 32);
        assert_eq!(rhs.len(), 32);
        assert_eq!(lhs.raw(), [0u8; 32]);
        assert_eq!(rhs.raw(), [0u8; 32]);
    }

    #[test]
    fn concat() {
        let lhs = H512::new([0u8; 64]);
        let rhs = H512::new([0u8; 64]);
        let concatenated = lhs.concat(rhs);
        assert_eq!(concatenated.len(), 128);
        assert_eq!(concatenated.raw(), [0u8; 128]);
    }

    #[test]
    fn from_slice() {
        let bytes = H512::from_slice(&[0u8; 64]);
        assert_eq!(bytes, [0u8; 64]);
    }

    #[test]
    #[should_panic(expected = "bad conversion: expected 64 bytes, got 3 bytes")]
    fn from_slice_panic() {
        let slice: &[u8] = &[1, 2, 3];
        let _ = H512::from_slice(slice);
    }

    #[test]
    fn random() {
        let bytes = H512::random();
        assert_ne!(bytes, H512::zero());
        assert_eq!(bytes.len(), 64);
    }

    #[test]
    fn random_with() {
        let mut rng = rand_core::OsRng;
        let bytes = H512::random_with(&mut rng);
        assert_ne!(bytes, H512::zero());
        assert_eq!(bytes.len(), 64);
    }

    #[test]
    fn iter() {
        let bytes = H512::new([0u8; 64]);
        let copy: Vec<u8> = bytes.iter().map(|i| i + 1).collect();
        assert_eq!(copy, [1u8; 64]);
    }

    #[test]
    fn iter_mut() {
        let mut bytes = H512::new([0u8; 64]);
        bytes.iter_mut().for_each(|i| *i = 1);
        assert_eq!(bytes, [1u8; 64]);
    }

    #[test]
    fn fmt_display() {
        let bytes = H512::new([0xFF; 64]);
        let fmt = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
        assert_eq!(f!("{bytes}"), fmt);
    }

    #[test]
    fn fmt_debug() {
        let bytes = H512::new([0xFF; 64]);
        let fmt = "H512(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff)";
        assert_eq!(f!("{bytes:?}"), fmt);
    }

    #[test]
    fn fmt_hex_lowercase() {
        let bytes = H512::new([0xFF; 64]);
        let fmt = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
        assert_eq!(f!("{bytes:x}"), fmt);
    }

    #[test]
    fn fmt_hex_uppercase() {
        let bytes = H512::new([0xFF; 64]);
        let fmt = "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        assert_eq!(f!("{bytes:X}"), fmt);
    }

    #[test]
    fn from_str() {
        let str = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
        let bytes = H512::from_str(str).unwrap();
        assert_eq!(f!("{bytes}"), str);
    }

    #[test]
    fn from_hex_lowercase() {
        let str = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
        let bytes = H512::from_hex(str).unwrap();
        assert_eq!(f!("{bytes:x}"), str);
    }

    #[test]
    fn from_hex_uppercase() {
        let str = "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        let bytes = H512::from_hex(str).unwrap();
        assert_eq!(f!("{bytes:X}"), str);
    }

    #[test]
    fn serde_json() {
        let bytes = H512::new([0xFF; 64]);
        let json = serde::to_string(&bytes).unwrap();
        assert_eq!(json, "\"0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"");
        let de: H512 = serde::from_str(&json).unwrap();
        assert_eq!(de, bytes);
    }

    #[test]
    #[rustfmt::skip]
    fn serde_value() {
        let bytes = H512::new([0xFF; 64]);
        let ser = serde::to_value(&bytes).unwrap();
        assert_eq!(ser, json! { "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff" });
        let de: H512 = serde::from_value(ser).unwrap();
        assert_eq!(de, bytes);
    }
}
