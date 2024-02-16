// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use stacks_rs::clarity;
use stacks_rs::derive::FromTuple;

#[test]
fn test_derive_from_tuple() {
    #[derive(FromTuple)]
    struct Payload {
        #[stacks(key = "data")]
        data: Data,
        #[stacks(key = "meta")]
        meta: Meta,
    }

    #[derive(FromTuple)]
    struct Data {
        #[stacks(key = "a")]
        a: i128,
        #[stacks(key = "b")]
        b: u128,
        #[stacks(key = "c")]
        c: Vec<u8>,
        #[stacks(key = "d")]
        d: String,
    }

    #[derive(FromTuple)]
    struct Meta {
        #[stacks(key = "e")]
        e: bool,
        #[stacks(key = "f")]
        f: bool,
    }

    let data = clarity!(
        Tuple,
        ("a", clarity!(Int, 1)),
        ("b", clarity!(UInt, 1)),
        ("c", clarity!(Buffer, vec![0x01, 0x02, 0x03])),
        ("d", clarity!(PrincipalStandard, "STX000001"))
    );

    let meta = clarity!(Tuple, ("e", clarity!(True)), ("f", clarity!(False)));
    let tuple = clarity!(Tuple, ("data", data), ("meta", meta));

    let parsed = Payload::try_from(tuple).unwrap();
    assert_eq!(parsed.data.a, 1);
    assert_eq!(parsed.data.b, 1);
    assert_eq!(parsed.data.c, vec![0x01, 0x02, 0x03]);
    assert_eq!(parsed.data.d, "STX000001");
    assert_eq!(parsed.meta.e, true);
    assert_eq!(parsed.meta.f, false);
}

#[test]
fn test_derive_from_tuple_to_string() {
    #[derive(FromTuple)]
    struct Payload {
        #[stacks(key = "int")]
        int: String,
        #[stacks(key = "uint")]
        uint: String,
        #[stacks(key = "buffer")]
        buffer: String,
        #[stacks(key = "true")]
        _true: String,
        #[stacks(key = "false")]
        _false: String,
        #[stacks(key = "p_s")]
        principal_standard: String,
        #[stacks(key = "p_c")]
        principal_contract: String,
        #[stacks(key = "response_ok")]
        response_ok: String,
        #[stacks(key = "response_err")]
        response_err: String,
        #[stacks(key = "optional_some")]
        optional_some: String,
        #[stacks(key = "optional_none")]
        optional_none: String,
        #[stacks(key = "list")]
        list: String,
        #[stacks(key = "tuple")]
        tuple: String,
        #[stacks(key = "string_ascii")]
        string_ascii: String,
        #[stacks(key = "string_utf8")]
        string_utf8: String,
    }

    let data = clarity!(
        Tuple,
        ("int", clarity!(Int, 1)),
        ("uint", clarity!(UInt, 1)),
        ("buffer", clarity!(Buffer, vec![0x01, 0x02, 0x03])),
        ("true", clarity!(True)),
        ("false", clarity!(False)),
        ("p_s", clarity!(PrincipalStandard, "STX000001")),
        ("p_c", clarity!(PrincipalContract, "STX000001", "contract")),
        ("response_ok", clarity!(ResponseOk, clarity!(Int, 1))),
        ("response_err", clarity!(ResponseErr, clarity!(Int, 1))),
        ("optional_some", clarity!(OptionalSome, clarity!(Int, 1))),
        ("optional_none", clarity!(OptionalNone)),
        ("list", clarity!(List, clarity!(Int, 1), clarity!(UInt, 1))),
        (
            "tuple",
            clarity!(Tuple, ("hello", clarity!(Int, 1)), ("x", clarity!(UInt, 2)))
        ),
        ("string_ascii", clarity!(StringAscii, "hello world")),
        ("string_utf8", clarity!(StringUtf8, "hello \u{1234}"))
    );

    let parsed = Payload::try_from(data).unwrap();

    assert_eq!(parsed.int, "1");
    assert_eq!(parsed.uint, "u1");
    assert_eq!(parsed.buffer, "0x010203");
    assert_eq!(parsed._true, "true");
    assert_eq!(parsed._false, "false");
    assert_eq!(parsed.principal_standard, "STX000001");
    assert_eq!(parsed.principal_contract, "STX000001.contract");
    assert_eq!(parsed.response_ok, "(ok 1)");
    assert_eq!(parsed.response_err, "(err 1)");
    assert_eq!(parsed.optional_some, "(some 1)");
    assert_eq!(parsed.optional_none, "none");
    assert_eq!(parsed.list, "(list 1 u1)");
    assert_eq!(parsed.tuple, "(tuple (hello 1) (x u2))");
    assert_eq!(parsed.string_ascii, "\"hello world\"");
    assert_eq!(parsed.string_utf8, "u\"hello ሴ\"");
}
