// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use stacks_rs::clarity;
use stacks_rs::clarity::Any;
use stacks_rs::clarity::Cast;
use stacks_rs::clarity::Int;
use stacks_rs::clarity::OptionalSome;
use stacks_rs::clarity::ResponseErr;
use stacks_rs::clarity::ResponseOk;
use stacks_rs::clarity::Tuple;
use stacks_rs::derive;
use stacks_rs::derive::FromTuple;

#[test]
fn test_derive_from_tuple() {
    #[derive(FromTuple)]
    struct Payload {
        #[stacks(key = "data")]
        data: Data,
        #[stacks(key = "meta")]
        meta: Meta,
        #[stacks(key = "response")]
        response: Response,
        #[stacks(key = "options")]
        options: Options,
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

    #[derive(FromTuple)]
    struct Options {
        #[stacks(key = "g")]
        g: Option<i128>,
        #[stacks(key = "h")]
        h: Option<i128>,
        #[stacks(key = "i")]
        i: Option<u128>,
        #[stacks(key = "j")]
        j: Option<u128>,
        #[stacks(key = "k")]
        k: Option<String>,
        #[stacks(key = "l")]
        l: Option<String>,
    }

    #[derive(FromTuple)]
    struct Response {
        #[stacks(key = "m", response)]
        m: u128,
        #[stacks(key = "n", response)]
        n: i128,
    }

    let data = clarity!(
        Tuple,
        ("a", clarity!(Int, 1)),
        ("b", clarity!(UInt, 1)),
        ("c", clarity!(Buffer, vec![0x01, 0x02, 0x03])),
        ("d", clarity!(PrincipalStandard, "STX000001"))
    );

    let meta = clarity!(Tuple, ("e", clarity!(True)), ("f", clarity!(False)));

    let optionals = clarity!(
        Tuple,
        ("g", clarity!(OptionalSome, clarity!(Int, 1))),
        ("h", clarity!(OptionalNone)),
        ("i", clarity!(OptionalSome, clarity!(UInt, 1))),
        ("j", clarity!(OptionalNone)),
        (
            "k",
            clarity!(
                OptionalSome,
                clarity!(PrincipalContract, "STX000001", "contract")
            )
        ),
        ("l", clarity!(OptionalNone))
    );

    let response = clarity!(
        Tuple,
        ("m", clarity!(ResponseOk, clarity!(UInt, 1))),
        ("n", clarity!(ResponseOk, clarity!(Int, 1)))
    );

    let tuple = clarity!(
        Tuple,
        ("data", data),
        ("meta", meta),
        ("response", response),
        ("options", optionals)
    );

    let parsed = Payload::try_from(tuple).unwrap();
    assert_eq!(parsed.data.a, 1);
    assert_eq!(parsed.data.b, 1);
    assert_eq!(parsed.data.c, vec![0x01, 0x02, 0x03]);
    assert_eq!(parsed.data.d, "STX000001");
    assert_eq!(parsed.meta.e, true);
    assert_eq!(parsed.meta.f, false);
    assert_eq!(parsed.options.g, Some(1));
    assert_eq!(parsed.options.h, None);
    assert_eq!(parsed.options.i, Some(1));
    assert_eq!(parsed.options.j, None);
    assert_eq!(parsed.options.k, Some("STX000001.contract".to_string()));
    assert_eq!(parsed.options.l, None);
    assert_eq!(parsed.response.m, 1);
    assert_eq!(parsed.response.n, 1);

    // assert_eq!(parsed.meta.g, Some(1));
    // assert_eq!(parsed.meta.h, None);
    // assert_eq!(parsed.meta.i, Some(1));
    // assert_eq!(parsed.meta.j, None);
    // assert_eq!(parsed.response.k, 1);
    // assert_eq!(parsed.response.l, 1);
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
