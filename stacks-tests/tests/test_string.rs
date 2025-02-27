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
use stacks_primitives::codec::Decode;
use stacks_primitives::codec::Encode;
use stacks_primitives::hex::FromHex;
use stacks_primitives::string::BoundedString;
use stacks_primitives::string::Constraint;
use stacks_primitives::string::Error;
use stacks_primitives::string::Identifier;
use stacks_primitives::string::Memo;

mod bounded_string {
    use super::*;

    #[derive(PartialEq, Eq)]
    struct Uppercase;
    impl Constraint for Uppercase {
        fn assert(str: &str) -> bool {
            str.chars().all(|c| c.is_uppercase())
        }
    }

    #[test]
    fn new_min() {
        let str = BoundedString::<13, 100>::new("random string").unwrap();
        assert_eq!(str, "random string");
        assert_eq!(str.len(), 13);
    }

    #[test]
    fn new_max() {
        let str = BoundedString::<0, 13>::new("random string").unwrap();
        assert_eq!(str, "random string");
        assert_eq!(str.len(), 13);
    }

    #[test]
    fn new_constraint() {
        let str = BoundedString::<0, 100, Uppercase>::new("RANDOM").unwrap();
        assert_eq!(str, "RANDOM");
        assert_eq!(str.len(), 6);
    }

    #[test]
    fn new_empty() {
        let str = BoundedString::<0, 13>::new("").unwrap();
        assert_eq!(str.len(), 0);
        assert_eq!(str, "");
    }

    #[test]
    fn new_err_overflow() {
        let result = BoundedString::<0, 10>::new("random string");
        assert_eq!(result, Err(Error::Overflow(13, 10)))
    }

    #[test]
    fn new_err_underflow() {
        let result = BoundedString::<100, 100>::new("random string");
        assert_eq!(result, Err(Error::Underflow(13, 100)))
    }

    #[test]
    fn new_err_violation() {
        let result = BoundedString::<0, 13, Uppercase>::new("random string");
        assert_eq!(result, Err(Error::Violation("random string".into())))
    }

    #[test]
    fn fmt_display() {
        let str = BoundedString::<0, 100>::new("random string").unwrap();
        let fmt = "random string";
        assert_eq!(f!("{str}"), fmt);
    }

    #[test]
    fn fmt_debug() {
        let str = BoundedString::<0, 100>::new("random string").unwrap();
        let fmt = "BoundedString(random string)";
        assert_eq!(f!("{str:?}"), fmt);
    }

    #[test]
    fn fmt_hex_lowercase() {
        let str = BoundedString::<0, 100>::new("random string").unwrap();
        let fmt = "0x72616e646f6d20737472696e67";
        assert_eq!(f!("{str:x}"), fmt);
    }

    #[test]
    fn fmt_hex_uppercase() {
        let str = BoundedString::<0, 100>::new("random string").unwrap();
        let fmt = "0x72616E646F6D20737472696E67";
        assert_eq!(f!("{str:X}"), fmt);
    }

    #[test]
    fn from_str() {
        let str = BoundedString::<0, 100>::from_str("random string").unwrap();
        assert_eq!(str, "random string");
        assert_eq!(str.len(), 13);
    }

    #[test]
    fn from_hex_lowercase() {
        let hex = "0x72616e646f6d20737472696e67";
        let str = BoundedString::<0, 100>::from_hex(hex).unwrap();
        assert_eq!(str, "random string");
        assert_eq!(str.len(), 13);
    }

    #[test]
    fn from_hex_uppercase() {
        let hex = "0x72616E646F6D20737472696E67";
        let str = BoundedString::<0, 100>::from_hex(hex).unwrap();
        assert_eq!(str, "random string");
        assert_eq!(str.len(), 13);
    }

    #[test]
    fn serde_json() {
        let str = BoundedString::<0, 100>::new("random string").unwrap();
        let json = serde::to_string(&str).unwrap();
        assert_eq!(json, "\"random string\"");
        let de: BoundedString<0, 100> = serde::from_str(&json).unwrap();
        assert_eq!(de, str);
    }

    #[test]
    fn serde_value() {
        let str = BoundedString::<0, 100>::new("random string").unwrap();
        let ser = serde::to_value(&str).unwrap();
        assert_eq!(ser, json! { "random string" });
        let de: BoundedString<0, 100> = serde::from_value(ser).unwrap();
        assert_eq!(de, str);
    }
}

mod identifier {
    use super::*;

    #[test]
    fn new_min() {
        let str = Identifier::new("a").unwrap();
        assert_eq!(str.len(), 1);
        assert_eq!(str, "a");
    }

    #[test]
    fn new_max() {
        let str = Identifier::new("a".repeat(128)).unwrap();
        assert_eq!(str, "a".repeat(128));
        assert_eq!(str.len(), 128);
    }

    #[test]
    fn new_err_overflow() {
        let result = Identifier::new("a".repeat(129));
        assert_eq!(result, Err(Error::Overflow(129, 128)));
    }

    #[test]
    fn new_err_violation_start() {
        let result = Identifier::new("1-my-func");
        assert_eq!(result, Err(Error::Violation("1-my-func".into())));
    }

    #[test]
    fn new_err_violation_chars() {
        let result = Identifier::new("my-#func");
        assert_eq!(result, Err(Error::Violation("my-#func".into())));
    }

    #[test]
    fn new_err_violation_space() {
        let result = Identifier::new("my func");
        assert_eq!(result, Err(Error::Violation("my func".into())));
    }

    #[test]
    fn new_err_violation_empty() {
        let result = Identifier::new("");
        assert_eq!(result, Err(Error::Violation("".into())));
    }

    #[test]
    fn fmt_display() {
        let str = Identifier::new("my-random-function").unwrap();
        let fmt = "my-random-function";
        assert_eq!(f!("{str}"), fmt);
    }

    #[test]
    fn fmt_debug() {
        let str = Identifier::new("my-random-function").unwrap();
        let fmt = "Identifier(my-random-function)";
        assert_eq!(f!("{str:?}"), fmt);
    }

    #[test]
    fn fmt_hex_lowercase() {
        let str = Identifier::new("my-random-function").unwrap();
        let fmt = "0x126d792d72616e646f6d2d66756e6374696f6e";
        assert_eq!(f!("{str:x}"), fmt);
    }

    #[test]
    fn fmt_hex_uppercase() {
        let str = Identifier::new("my-random-function").unwrap();
        let fmt = "0x126D792D72616E646F6D2D66756E6374696F6E";
        assert_eq!(f!("{str:X}"), fmt);
    }

    #[test]
    fn from_str() {
        let str = Identifier::from_str("my-random-function").unwrap();
        assert_eq!(str, "my-random-function");
        assert_eq!(str.len(), 18);
    }

    #[test]
    fn from_hex_lowercase() {
        let hex = "0x126d792d72616e646f6d2d66756e6374696f6e";
        let str = Identifier::from_hex(hex).unwrap();
        assert_eq!(str, "my-random-function");
        assert_eq!(str.len(), 18);
    }

    #[test]
    fn from_hex_uppercase() {
        let hex = "0x126D792D72616E646F6D2D66756E6374696F6E";
        let str = Identifier::from_hex(hex).unwrap();
        assert_eq!(str, "my-random-function");
        assert_eq!(str.len(), 18);
    }

    #[test]
    fn serde_json() {
        let str = Identifier::new("my-random-function").unwrap();
        let json = serde::to_string(&str).unwrap();
        assert_eq!(json, "\"my-random-function\"");
        let de: Identifier = serde::from_str(&json).unwrap();
        assert_eq!(de, str);
    }

    #[test]
    fn serde_value() {
        let str = Identifier::new("my-random-function").unwrap();
        let ser = serde::to_value(&str).unwrap();
        assert_eq!(ser, json! { "my-random-function" });
        let de: Identifier = serde::from_value(ser).unwrap();
        assert_eq!(de, str);
    }

    #[test]
    fn encode() {
        let str = Identifier::new("my-func").unwrap();
        let bytes = str.encode().unwrap();
        assert_eq!(&bytes[..], b"\x07my-func");
    }

    #[test]
    fn decode() {
        let str = Identifier::new("my-func").unwrap();
        let mut bytes = Bytes::from_slice(b"\x07my-func");
        assert_eq!(str, Identifier::decode(&mut bytes).unwrap())
    }

    #[test]
    fn decode_err_tag() {
        let mut bytes = Bytes::new();
        let result = Identifier::decode(&mut bytes);
        let msg = "Not enough bytes to decode 'Identifier.length': expected at least '1' bytes, found '0' bytes";
        assert_eq!(result, Err(Error::Codec(msg.into())))
    }

    #[test]
    fn decode_err_content() {
        let mut bytes = Bytes::from(vec![5, b'a', b'b', b'c']);
        let result = Identifier::decode(&mut bytes);
        let msg = "Not enough bytes to decode 'Identifier.content': expected at least '5' bytes, found '3' bytes";
        assert_eq!(result, Err(Error::Codec(msg.into())))
    }

    #[test]
    fn decode_err_excess_bytes() {
        let mut bytes = Bytes::from(vec![3, b'a', b'b', b'c', b'd', b'e']);
        let result = Identifier::decode(&mut bytes);
        let type_name = std::any::type_name::<Identifier>();
        let msg = f!("Extra bytes remain after decoding '{type_name}': found '2' unexpected bytes");
        assert_eq!(result, Err(Error::Codec(msg.into())))
    }
}

mod memo {
    use super::*;

    #[test]
    fn new_min() {
        let str = Memo::new("a").unwrap();
        assert_eq!(str.len(), 1);
        assert_eq!(str, "a");
    }

    #[test]
    fn new_max() {
        let str = Memo::new("a".repeat(34)).unwrap();
        assert_eq!(str, "a".repeat(34));
        assert_eq!(str.len(), 34);
    }

    #[test]
    fn new_err_overflow() {
        let result = Memo::new("a".repeat(35));
        assert_eq!(result, Err(Error::Overflow(35, 34)));
    }

    #[test]
    fn fmt_display() {
        let str = Memo::new("my random memo").unwrap();
        let fmt = "my random memo";
        assert_eq!(f!("{str}"), fmt);
    }

    #[test]
    fn fmt_debug() {
        let str = Memo::new("my random memo").unwrap();
        let fmt = "Memo(my random memo)";
        assert_eq!(f!("{str:?}"), fmt);
    }

    #[test]
    fn fmt_hex_lowercase() {
        let str = Memo::new("my random memo").unwrap();
        let fmt = "0x0e6d792072616e646f6d206d656d6f";
        assert_eq!(f!("{str:x}"), fmt);
    }

    #[test]
    fn fmt_hex_uppercase() {
        let str = Memo::new("my random memo").unwrap();
        let fmt = "0x0E6D792072616E646F6D206D656D6F";
        assert_eq!(f!("{str:X}"), fmt);
    }

    #[test]
    fn from_str() {
        let str = Memo::from_str("my random memo").unwrap();
        assert_eq!(str, "my random memo");
        assert_eq!(str.len(), 14);
    }

    #[test]
    fn from_hex_lowercase() {
        let hex = "0x0e6d792072616e646f6d206d656d6f";
        let str = Memo::from_hex(hex).unwrap();
        assert_eq!(str, "my random memo");
        assert_eq!(str.len(), 14);
    }

    #[test]
    fn from_hex_uppercase() {
        let hex = "0x0E6D792072616E646F6D206D656D6F";
        let str = Memo::from_hex(hex).unwrap();
        assert_eq!(str, "my random memo");
        assert_eq!(str.len(), 14);
    }

    #[test]
    fn serde_json() {
        let str = Memo::new("my random memo").unwrap();
        let json = serde::to_string(&str).unwrap();
        assert_eq!(json, "\"my random memo\"");
        let de: Memo = serde::from_str(&json).unwrap();
        assert_eq!(de, str);
    }

    #[test]
    fn serde_value() {
        let str = Memo::new("my random memo").unwrap();
        let ser = serde::to_value(&str).unwrap();
        assert_eq!(ser, json! { "my random memo" });
        let de: Memo = serde::from_value(ser).unwrap();
        assert_eq!(de, str);
    }

    #[test]
    fn encode() {
        let str = Memo::new("my random memo").unwrap();
        let bytes = str.encode().unwrap();
        assert_eq!(&bytes[..], b"\x0Emy random memo");
    }

    #[test]
    fn decode() {
        let str = Memo::new("my random memo").unwrap();
        let mut bytes = Bytes::from_slice(b"\x0Emy random memo");
        assert_eq!(str, Memo::decode(&mut bytes).unwrap())
    }

    #[test]
    fn decode_err_tag() {
        let mut bytes = Bytes::new();
        let result = Memo::decode(&mut bytes);
        let msg = "Not enough bytes to decode 'Memo.length': expected at least '1' bytes, found '0' bytes";
        assert_eq!(result, Err(Error::Codec(msg.into())))
    }

    #[test]
    fn decode_err_content() {
        let mut bytes = Bytes::from(vec![5, b'a', b'b', b'c']);
        let result = Memo::decode(&mut bytes);
        let msg = "Not enough bytes to decode 'Memo.content': expected at least '5' bytes, found '3' bytes";
        assert_eq!(result, Err(Error::Codec(msg.into())))
    }

    #[test]
    fn decode_err_excess_bytes() {
        let mut bytes = Bytes::from(vec![3, b'a', b'b', b'c', b'd', b'e']);
        let result = Memo::decode(&mut bytes);
        let type_name = std::any::type_name::<Memo>();
        let msg = f!("Extra bytes remain after decoding '{type_name}': found '2' unexpected bytes");
        assert_eq!(result, Err(Error::Codec(msg.into())))
    }
}
