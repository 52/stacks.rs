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
use stacks_primitives::bytes::Bytes;
use stacks_primitives::bytes::FixedBytes;
use stacks_primitives::hex::FromHex;

mod fixed_bytes {
    use super::*;

    #[test]
    fn new() {
        let bytes = FixedBytes::new([0u8; 10]);
        assert_eq!(bytes, [0u8; 10])
    }

    #[test]
    fn zero() {
        let bytes = FixedBytes::<10>::zero();
        assert_eq!(bytes, [0u8; 10]);
    }

    #[test]
    fn repeat_byte() {
        let bytes = FixedBytes::<10>::repeat_byte(0xFF);
        assert_eq!(bytes, [0xFF; 10]);
    }

    #[test]
    fn len() {
        let bytes = FixedBytes::new([0u8; 10]);
        assert_eq!(bytes.len(), 10);
    }

    #[test]
    fn is_empty() {
        let bytes = FixedBytes::new([0u8; 10]);
        assert_eq!(bytes.is_empty(), false);
    }

    #[test]
    fn raw() {
        let bytes = FixedBytes::new([0u8; 10]);
        assert_eq!(bytes.raw(), [0u8; 10]);
    }

    #[test]
    fn append() {
        let bytes = FixedBytes::new([1, 2, 3]);
        let append = bytes.append::<4>(4);
        assert_eq!(append.len(), 4);
        assert_eq!(append.raw(), [1, 2, 3, 4]);
    }

    #[test]
    fn prepend() {
        let bytes = FixedBytes::new([1, 2, 3]);
        let prepended = bytes.prepend::<4>(0);
        assert_eq!(prepended.len(), 4);
        assert_eq!(prepended.raw(), [0, 1, 2, 3]);
    }

    #[test]
    fn split() {
        let bytes = FixedBytes::new([1, 2, 3, 4, 5, 6]);
        let (lhs, rhs) = bytes.split::<2, 4>();
        assert_eq!(lhs.len(), 2);
        assert_eq!(rhs.len(), 4);
        assert_eq!(lhs.raw(), [1, 2]);
        assert_eq!(rhs.raw(), [3, 4, 5, 6]);
    }

    #[test]
    fn concat() {
        let lhs = FixedBytes::new([1, 2, 3]);
        let rhs = FixedBytes::new([4, 5, 6]);
        let concatenated = lhs.concat::<6, 3>(rhs);
        assert_eq!(concatenated.len(), 6);
        assert_eq!(concatenated.raw(), [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn from_slice() {
        let bytes = FixedBytes::from_slice(&[0u8; 10]);
        assert_eq!(bytes, [0u8; 10])
    }

    #[test]
    fn random() {
        let bytes = FixedBytes::<100>::random();
        assert_ne!(bytes, FixedBytes::<100>::zero());
        assert_eq!(bytes.len(), 100);
    }

    #[test]
    fn random_with() {
        let mut rng = rand_core::OsRng;
        let bytes = FixedBytes::<100>::random_with(&mut rng);
        assert_ne!(bytes, FixedBytes::<100>::zero());
        assert_eq!(bytes.len(), 100);
    }

    #[test]
    fn iter() {
        let bytes = FixedBytes::new([0u8; 20]);
        let copy: Vec<u8> = bytes.iter().map(|i| i + 1).collect();
        assert_eq!(copy, [1u8; 20]);
    }

    #[test]
    fn iter_mut() {
        let mut bytes = FixedBytes::new([0u8; 20]);
        bytes.iter_mut().for_each(|i| *i = 1);
        assert_eq!(bytes, [1u8; 20]);
    }

    #[test]
    fn fmt_display() {
        let bytes = FixedBytes::new([0xFF; 20]);
        let fmt = "0xffffffffffffffffffffffffffffffffffffffff";
        assert_eq!(f!("{bytes}"), fmt);
    }

    #[test]
    fn fmt_debug() {
        let bytes = FixedBytes::new([0xFF; 20]);
        let fmt = "FixedBytes(0xffffffffffffffffffffffffffffffffffffffff)";
        assert_eq!(f!("{bytes:?}"), fmt);
    }

    #[test]
    fn fmt_hex_lowercase() {
        let bytes = FixedBytes::new([0xFF; 20]);
        let fmt = "0xffffffffffffffffffffffffffffffffffffffff";
        assert_eq!(f!("{bytes:x}"), fmt);
    }

    #[test]
    fn fmt_hex_uppercase() {
        let bytes = FixedBytes::new([0xFF; 20]);
        let fmt = "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        assert_eq!(f!("{bytes:X}"), fmt);
    }

    #[test]
    fn from_str() {
        let str = "0xffffffffffffffffffffffffffffffffffffffff";
        let bytes = FixedBytes::<20>::from_str(str).unwrap();
        assert_eq!(f!("{bytes}"), str);
    }

    #[test]
    fn from_hex_lowercase() {
        let str = "0xffffffffffffffffffffffffffffffffffffffff";
        let bytes = FixedBytes::<20>::from_hex(str).unwrap();
        assert_eq!(f!("{bytes:x}"), str);
    }

    #[test]
    fn from_hex_uppercase() {
        let str = "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        let bytes = FixedBytes::<20>::from_hex(str).unwrap();
        assert_eq!(f!("{bytes:X}"), str);
    }

    #[test]
    fn serde_json() {
        let bytes = FixedBytes::new([0xFF; 20]);
        let json = serde::to_string(&bytes).unwrap();
        assert_eq!(json, "\"0xffffffffffffffffffffffffffffffffffffffff\"");
        let de: FixedBytes<20> = serde::from_str(&json).unwrap();
        assert_eq!(de, bytes);
    }

    #[test]
    fn serde_value() {
        let bytes = FixedBytes::new([0xFF; 20]);
        let ser = serde::to_value(&bytes).unwrap();
        assert_eq!(ser, json! { "0xffffffffffffffffffffffffffffffffffffffff" });
        let de: FixedBytes<20> = serde::from_value(ser).unwrap();
        assert_eq!(de, bytes);
    }
}

mod bytes {
    use super::*;

    #[test]
    fn new() {
        let bytes = Bytes::new();
        assert_eq!(bytes.len(), 0);
    }

    #[test]
    fn from_static() {
        let bytes = Bytes::from_static(&[0u8; 10]);
        assert_eq!(bytes, [0u8; 10]);
    }

    #[test]
    fn from_slice() {
        let bytes = Bytes::from_slice(&[0u8; 10]);
        assert_eq!(bytes, [0u8; 10]);
    }

    #[test]
    fn len() {
        let bytes = Bytes::from_static(&[0u8; 10]);
        assert_eq!(bytes.len(), 10);
    }

    #[test]
    fn is_empty() {
        let bytes = Bytes::from_static(&[0u8; 10]);
        assert_eq!(bytes.is_empty(), false);
    }

    #[test]
    fn raw() {
        let bytes = Bytes::from_static(&[0u8; 10]);
        assert_eq!(bytes.raw(), Bytes::from([0u8; 10]));
    }

    #[test]
    fn iter() {
        let bytes = Bytes::from_static(&[0u8; 20]);
        let copy: Vec<u8> = bytes.iter().map(|i| i + 1).collect();
        assert_eq!(copy, [1u8; 20]);
    }

    #[test]
    fn fmt_display() {
        let bytes = Bytes::from_slice(&[0xFF; 10]);
        let fmt = "0xffffffffffffffffffff";
        assert_eq!(f!("{bytes}"), fmt);
    }

    #[test]
    fn fmt_debug() {
        let bytes = Bytes::from_slice(&[0xFF; 10]);
        let fmt = "Bytes(0xffffffffffffffffffff)";
        assert_eq!(f!("{bytes:?}"), fmt);
    }

    #[test]
    fn fmt_hex_lowercase() {
        let bytes = Bytes::from_slice(&[0xFF; 10]);
        let fmt = "0xffffffffffffffffffff";
        assert_eq!(f!("{bytes:x}"), fmt);
    }

    #[test]
    fn fmt_hex_uppercase() {
        let bytes = Bytes::from_slice(&[0xFF; 10]);
        let fmt = "0xFFFFFFFFFFFFFFFFFFFF";
        assert_eq!(f!("{bytes:X}"), fmt);
    }

    #[test]
    fn from_str() {
        let str = "0xffffffffffffffffffff";
        let bytes = Bytes::from_str(str).unwrap();
        assert_eq!(f!("{bytes}"), str);
    }

    #[test]
    fn from_hex_lowercase() {
        let str = "0xffffffffffffffffffff";
        let bytes = Bytes::from_hex(str).unwrap();
        assert_eq!(f!("{bytes:x}"), str);
    }

    #[test]
    fn from_hex_uppercase() {
        let str = "0xFFFFFFFFFFFFFFFFFFFF";
        let bytes = Bytes::from_hex(str).unwrap();
        assert_eq!(f!("{bytes:X}"), str);
    }

    #[test]
    fn serde_json() {
        let bytes = Bytes::from_static(&[0xFF; 20]);
        let json = serde::to_string(&bytes).unwrap();
        assert_eq!(json, "\"0xffffffffffffffffffffffffffffffffffffffff\"");
        let de: Bytes = serde::from_str(&json).unwrap();
        assert_eq!(de, bytes);
    }

    #[test]
    fn serde_value() {
        let bytes = Bytes::from_static(&[0xFF; 20]);
        let ser = serde::to_value(&bytes).unwrap();
        assert_eq!(ser, json! { "0xffffffffffffffffffffffffffffffffffffffff" });
        let de: Bytes = serde::from_value(ser).unwrap();
        assert_eq!(de, bytes);
    }
}
