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
use stacks_primitives::bytes::B256;
use stacks_primitives::bytes::B8;
use stacks_primitives::ecdsa::k256;
use stacks_primitives::ecdsa::Error;
use stacks_primitives::ecdsa::PrivateKey;
use stacks_primitives::ecdsa::PublicKey;
use stacks_primitives::ecdsa::Signature;
use stacks_primitives::hash::H256;
use stacks_primitives::hex::FromHex;

mod private_key {
    use super::*;

    #[test]
    fn new() {
        let k256 = k256::SigningKey::random(&mut rand_core::OsRng);
        let key = PrivateKey::new(k256.clone());
        assert_eq!(key.raw(), k256);
    }

    #[test]
    fn raw() {
        let k256 = k256::SigningKey::random(&mut rand_core::OsRng);
        let key = PrivateKey::new(k256.clone());
        assert_eq!(key.raw(), k256);
    }

    #[test]
    fn public() {
        let k256 = k256::SigningKey::random(&mut rand_core::OsRng);
        let key = PrivateKey::new(k256.clone());
        assert_eq!(key.public().raw(), *k256.verifying_key());
    }

    #[test]
    fn from_slice() {
        let k256 = k256::SigningKey::random(&mut rand_core::OsRng);
        let key = PrivateKey::from_slice(&k256.to_bytes()).unwrap();
        assert_eq!(key.raw(), k256);
    }

    #[test]
    #[should_panic]
    fn from_slice_panic() {
        let _ = PrivateKey::from_slice(&[0u8; 32]).unwrap();
    }

    #[test]
    fn from_bytes() {
        let k256 = k256::SigningKey::random(&mut rand_core::OsRng);
        let bytes = B256::from_slice(&k256.to_bytes());
        let key = PrivateKey::from_bytes(bytes).unwrap();
        assert_eq!(key.raw(), k256);
    }

    #[test]
    fn to_bytes() {
        let k256 = k256::SigningKey::random(&mut rand_core::OsRng);
        let bytes = B256::from_slice(&k256.to_bytes());
        let key = PrivateKey::from_bytes(bytes).unwrap();
        assert_eq!(bytes, key.to_bytes());
    }

    #[test]
    fn random() {
        let key = PrivateKey::random();
        assert_ne!(key.to_bytes(), B256::zero());
    }

    #[test]
    fn random_with() {
        let key = PrivateKey::random_with(&mut rand_core::OsRng);
        assert_ne!(key.to_bytes(), B256::zero());
    }

    #[test]
    fn sign() {
        let (key, prehash) = (PrivateKey::random(), H256::random());
        let signature = key.sign(prehash).unwrap();
        key.public().verify(prehash, signature).unwrap();
    }

    #[test]
    fn from_str() {
        let str = "0xf1f8c1bf124ba11f2e4515d27ea0175e5a0e09298cc0d70336c2574423bb447f";
        let key = PrivateKey::from_str(str).unwrap();
        let bytes = B256::from_str(str).unwrap();
        assert_eq!(key.to_bytes(), bytes);
    }

    #[test]
    fn from_hex_lowercase() {
        let str = "0xf1f8c1bf124ba11f2e4515d27ea0175e5a0e09298cc0d70336c2574423bb447f";
        let key = PrivateKey::from_hex(str).unwrap();
        let bytes = B256::from_str(str).unwrap();
        assert_eq!(key.to_bytes(), bytes);
    }

    #[test]
    fn from_hex_uppercase() {
        let str = "0xF1F8C1BF124BA11F2E4515D27EA0175E5A0E09298CC0D70336C2574423BB447F";
        let key = PrivateKey::from_hex(str).unwrap();
        let bytes = B256::from_str(str).unwrap();
        assert_eq!(key.to_bytes(), bytes);
    }
}

mod public_key {
    use super::*;

    #[test]
    fn new() {
        let k256 = k256::SigningKey::random(&mut rand_core::OsRng);
        let key = PublicKey::new(*k256.verifying_key());
        assert_eq!(key.raw(), *k256.verifying_key());
    }

    #[test]
    fn raw() {
        let k256 = k256::SigningKey::random(&mut rand_core::OsRng);
        let key = PublicKey::new(*k256.verifying_key());
        assert_eq!(key.raw(), *k256.verifying_key());
    }

    #[test]
    fn from_bytes() {
        let k256 = k256::SigningKey::random(&mut rand_core::OsRng);
        let point = k256.verifying_key().to_encoded_point(true);
        let key = PublicKey::from_bytes(&point.as_bytes()).unwrap();
        assert_eq!(key.raw(), *k256.verifying_key());
    }

    #[test]
    fn to_bytes_compressed() {
        let k256 = k256::SigningKey::random(&mut rand_core::OsRng);
        let point = k256.verifying_key().to_encoded_point(true);
        let key = PublicKey::new(*k256.verifying_key());
        assert_eq!(key.to_bytes(true), point.as_bytes());
    }

    #[test]
    fn to_bytes_uncompressed() {
        let k256 = k256::SigningKey::random(&mut rand_core::OsRng);
        let point = k256.verifying_key().to_encoded_point(false);
        let key = PublicKey::new(*k256.verifying_key());
        assert_eq!(key.to_bytes(false), point.as_bytes());
    }

    #[test]
    fn recover() {
        let (key, prehash) = (PrivateKey::random(), H256::random());
        let signature = key.sign(prehash).unwrap();
        let recovered = PublicKey::recover(prehash, signature).unwrap();
        assert_eq!(recovered.raw(), key.public().raw());
    }

    #[test]
    fn recover_s() {
        struct Case<'a> {
            key: &'a str,
            prehash: &'a str,
            compressed: &'a str,
            uncompressed: &'a str,
        }

        let cases = &[Case {
            key: "0xedf9aee84d9b7abc145504dde6726c64f369d37ee34ded868fabd876c26570bc",
            prehash: "0xb94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
            compressed: "0x03ef788b3830c00abe8f64f62dc32fc863bc0b2cafeb073b6c8e1c7657d9c2c3ab",
            uncompressed: "0x04ef788b3830c00abe8f64f62dc32fc863bc0b2cafeb073b6c8e1c7657d9c2c3ab5b435d20ea91337cdd8c30dd7427bb098a5355e9c9bfad43797899b8137237cf",
        }];

        for case in cases {
            let key = PrivateKey::from_hex(case.key).unwrap();
            let prehash = H256::from_hex(case.prehash).unwrap();
            let signature = key.sign(prehash).unwrap();
            let pkey = PublicKey::recover(prehash, signature).unwrap();
            assert_eq!(f!("{:x}", pkey.to_bytes(true)), case.compressed);
            assert_eq!(f!("{:x}", pkey.to_bytes(false)), case.uncompressed);
        }
    }

    #[test]
    fn verify() {
        struct Case<'a> {
            key: &'a str,
            signature: &'a str,
            prehash: &'a str,
            expected: fn(result: Result<(), Error>) -> bool,
        }

        let cases = &[
            Case {
                key: "0x0385f2e2867524289d6047d0d9c5e764c5d413729fc32291ad2c353fbc396a4219",
                signature: "0x00354445a1dc98a1bd27984dbe69979a5cd77886b4d9134af5c40e634d96e1cb445b97de5b632582d31704f86706a780886e6e381bfed65228267358262d203fe6",
                prehash: "0xb94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
                expected: |r| matches!(r, Ok(())),
            },
            Case {
                key: "0x0385f2e2867524289d6047d0d9c5e764c5d413729fc32291ad2c353fbc396a4219",
                signature: "0x00354445a1dc98a1bd27984dbe69979a5cd77886b4d9134af5c40e634d96e1cb445b97de5b632582d31704f86706a780886e6e381bfed65228267358262d203fe6",
                prehash: "0xca3704aa0b06f5954c79ee837faa152d84d6b2d42838f0637a15eda8337dbdce",
                expected: |r| matches!(r, Err(Error { .. })),
            },
            Case {
                key: "0x034c35b09b758678165d6ed84a50b329900c99986cf8e9a358ceae0d03af91f5b6",
                signature: "0x00354445a1dc98a1bd27984dbe69979a5cd77886b4d9134af5c40e634d96e1cb445b97de5b632582d31704f86706a780886e6e381bfed65228267358262d203fe6",
                prehash: "0xb94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
                expected: |r| matches!(r, Err(Error { .. })),
            },
            Case {
                key: "0x0385f2e2867524289d6047d0d9c5e764c5d413729fc32291ad2c353fbc396a4219",
                signature: "0x00354445a1dc98a1bd27984dbe69979a5cd77886b4d9134af5c40e634d96e1cb445b97de5b632582d31704f86706a780886e6e381bfed65228267358262d203fe7",
                prehash: "0xb94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
                expected: |r| matches!(r, Err(Error { .. })),
            },
            Case {
                key: "0x0385f2e2867524289d6047d0d9c5e764c5d413729fc32291ad2c353fbc396a4219",
                signature: "0x00454445a1dc98a1bd27984dbe69979a5cd77886b4d9134af5c40e634d96e1cb445b97de5b632582d31704f86706a780886e6e381bfed65228267358262d203fe6",
                prehash: "0xb94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
                expected: |r| matches!(r, Err(Error { .. })),
            },
            Case {
                key: "0x0385f2e2867524289d6047d0d9c5e764c5d413729fc32291ad2c353fbc396a4219",
                signature: "0x01354445a1dc98a1bd27984dbe69979a5cd77886b4d9134af5c40e634d96e1cb445b97de5b632582d31704f86706a780886e6e381bfed65228267358262d203fe6",
                prehash: "0xb94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
                expected: |r| matches!(r, Err(Error { .. })),
            },
            Case {
                key: "0x0385f2e2867524289d6047d0d9c5e764c5d413729fc32291ad2c353fbc396a4219",
                signature: "0x02354445a1dc98a1bd27984dbe69979a5cd77886b4d9134af5c40e634d96e1cb445b97de5b632582d31704f86706a780886e6e381bfed65228267358262d203fe6",
                prehash: "0xb94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
                expected: |r| matches!(r, Err(Error { .. })),
            },
            Case {
                key: "0x0385f2e2867524289d6047d0d9c5e764c5d413729fc32291ad2c353fbc396a4219",
                signature: "0x03354445a1dc98a1bd27984dbe69979a5cd77886b4d9134af5c40e634d96e1cb445b97de5b632582d31704f86706a780886e6e381bfed65228267358262d203fe6",
                prehash: "0xb94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
                expected: |r| matches!(r, Err(Error { .. })),
            },
            Case {
                key: "0x02ba024f120e212fa889c0b34986604ef7cab1834c86e8d78227c2874c74a7548d",
                signature: "0x01825e3c0eb5285f2f1d0f4b8f9cb9def69ba50303bb2f8fcbaf7ee0f9ff1beac781373cdb30792c600c2d1af681e822a90c8fc8ee2df1fe15719f0833c9e45c57",
                prehash: "0x2d072e745f77c6d563ccff3a6686303db2e56b250f44d4334a9b5cf10bde63fb",
                expected: |r| matches!(r, Err(Error { .. }))
            }
        ];

        for case in cases {
            let key = PublicKey::from_hex(case.key).unwrap();
            let signature = Signature::from_hex(case.signature).unwrap();
            let prehash = H256::from_hex(case.prehash).unwrap();

            assert_eq!(key.to_string(), case.key);
            assert_eq!(signature.to_string(), case.signature);

            let result = key.verify(prehash, signature);
            assert!((case.expected)(result))
        }
    }

    #[test]
    fn derive() {
        struct Case<'a> {
            key: &'a str,
            compressed: &'a str,
            uncompressed: &'a str,
        }

        let cases = &[Case {
            key: "0xedf9aee84d9b7abc145504dde6726c64f369d37ee34ded868fabd876c26570bc",
            compressed: "0x03ef788b3830c00abe8f64f62dc32fc863bc0b2cafeb073b6c8e1c7657d9c2c3ab",
            uncompressed: "0x04ef788b3830c00abe8f64f62dc32fc863bc0b2cafeb073b6c8e1c7657d9c2c3ab5b435d20ea91337cdd8c30dd7427bb098a5355e9c9bfad43797899b8137237cf",
        }];

        for case in cases {
            let key = PrivateKey::from_hex(case.key).unwrap().public();
            assert_eq!(f!("{:x}", key.to_bytes(true)), case.compressed);
            assert_eq!(f!("{:x}", key.to_bytes(false)), case.uncompressed);
        }
    }

    #[test]
    fn fmt_display() {
        let str = "0xf1f8c1bf124ba11f2e4515d27ea0175e5a0e09298cc0d70336c2574423bb447f";
        let key = PrivateKey::from_hex(str).unwrap().public();
        let fmt = "0x039de766f21c5c501d93b82e6519ea1336b2858494e77cd3a5f1efafe859769a27";
        assert_eq!(f!("{key}"), fmt);
    }

    #[test]
    fn fmt_debug() {
        let str = "0xf1f8c1bf124ba11f2e4515d27ea0175e5a0e09298cc0d70336c2574423bb447f";
        let key = PrivateKey::from_hex(str).unwrap().public();
        let fmt = "PublicKey(0x039de766f21c5c501d93b82e6519ea1336b2858494e77cd3a5f1efafe859769a27)";
        assert_eq!(f!("{key:?}"), fmt);
    }

    #[test]
    fn fmt_hex_lowercase() {
        let str = "0xf1f8c1bf124ba11f2e4515d27ea0175e5a0e09298cc0d70336c2574423bb447f";
        let key = PrivateKey::from_hex(str).unwrap().public();
        let fmt = "0x039de766f21c5c501d93b82e6519ea1336b2858494e77cd3a5f1efafe859769a27";
        assert_eq!(f!("{key:x}"), fmt);
    }

    #[test]
    fn fmt_hex_uppercase() {
        let str = "0xf1f8c1bf124ba11f2e4515d27ea0175e5a0e09298cc0d70336c2574423bb447f";
        let key = PrivateKey::from_hex(str).unwrap().public();
        let fmt = "0x039DE766F21C5C501D93B82E6519EA1336B2858494E77CD3A5F1EFAFE859769A27";
        assert_eq!(f!("{key:X}"), fmt);
    }

    #[test]
    fn from_str() {
        let str = "0x039de766f21c5c501d93b82e6519ea1336b2858494e77cd3a5f1efafe859769a27";
        let key = PublicKey::from_str(str).unwrap();
        let bytes = Bytes::from_str(str).unwrap();
        assert_eq!(key.to_bytes(true), &bytes[..]);
    }

    #[test]
    fn from_hex_lowercase() {
        let str = "0x039de766f21c5c501d93b82e6519ea1336b2858494e77cd3a5f1efafe859769a27";
        let key = PublicKey::from_hex(str).unwrap();
        let bytes = Bytes::from_hex(str).unwrap();
        assert_eq!(key.to_bytes(true), &bytes[..]);
    }

    #[test]
    fn from_hex_uppercase() {
        let str = "0x039DE766F21C5C501D93B82E6519EA1336B2858494E77CD3A5F1EFAFE859769A27";
        let key = PublicKey::from_hex(str).unwrap();
        let bytes = Bytes::from_hex(str).unwrap();
        assert_eq!(key.to_bytes(true), &bytes[..]);
    }

    #[test]
    fn serde_json() {
        let str = "0x039de766f21c5c501d93b82e6519ea1336b2858494e77cd3a5f1efafe859769a27";
        let key = PublicKey::from_hex(str).unwrap();
        let json = serde::to_string(&key).unwrap();
        assert_eq!(json, "\"0x039de766f21c5c501d93b82e6519ea1336b2858494e77cd3a5f1efafe859769a27\"");
        let de: PublicKey = serde::from_str(&json).unwrap();
        assert_eq!(de, key);
    }

    #[test]
    fn serde_value() {
        let str = "0x039de766f21c5c501d93b82e6519ea1336b2858494e77cd3a5f1efafe859769a27";
        let key = PublicKey::from_hex(str).unwrap();
        let ser = serde::to_value(&key).unwrap();
        assert_eq!(ser, json! { str });
        let de: PublicKey = serde::from_value(ser).unwrap();
        assert_eq!(de, key);
    }
}

mod signature {
    use super::*;

    #[test]
    fn new() {
        let (r, s, v) = (B256::random(), B256::random(), B8::random());
        let sig = Signature::new(r, s, v);
        assert_eq!(sig.r(), r);
        assert_eq!(sig.s(), s);
        assert_eq!(sig.v(), v);
    }

    #[test]
    fn r() {
        let (r, s, v) = (B256::random(), B256::random(), B8::random());
        let sig = Signature::new(r, s, v);
        assert_eq!(sig.r(), r);
    }

    #[test]
    fn s() {
        let (r, s, v) = (B256::random(), B256::random(), B8::random());
        let sig = Signature::new(r, s, v);
        assert_eq!(sig.s(), s);
    }

    #[test]
    fn v() {
        let (r, s, v) = (B256::random(), B256::random(), B8::random());
        let sig = Signature::new(r, s, v);
        assert_eq!(sig.v(), v);
    }

    #[test]
    fn zero() {
        let (r, s, v) = (B256::zero(), B256::zero(), B8::zero());
        let sig = Signature::new(r, s, v);
        assert_eq!(sig, Signature::zero());
    }

    #[test]
    fn from_bytes() {
        let bytes = FixedBytes::<65>::random();
        let sig = Signature::from_bytes(bytes);
        assert_eq!(sig.to_bytes(), bytes);
    }

    #[test]
    fn to_bytes() {
        let bytes = FixedBytes::<65>::random();
        let sig = Signature::from_bytes(bytes);
        assert_eq!(sig.to_bytes(), bytes);
    }

    #[test]
    fn from_k256() {
        let (r, s, v) = (B256::random(), B256::random(), B8::from(0));
        let sig = Signature::new(r, s, v);
        let (ks, kr) = sig.to_k256().unwrap();
        assert_eq!(sig, Signature::from_k256(ks, kr));
    }

    #[test]
    fn to_k256() {
        let (r, s, v) = (B256::random(), B256::random(), B8::from(0));
        let sig = Signature::new(r, s, v);
        let (ks, kr) = sig.to_k256().unwrap();
        assert_eq!(sig, Signature::from_k256(ks, kr));
    }

    #[test]
    fn normalize_s() {
        let (key, prehash) = (PrivateKey::random(), H256::random());
        let sig = key.sign(prehash).unwrap();
        let normalized = sig.normalize_s();
        assert_eq!(normalized, sig);
    }

    #[test]
    fn normalize_s_high() {
        use k256::Curve;
        use k256::Encoding;
        use k256::Secp256k1;
        use k256::U256;

        let (key, prehash) = (PrivateKey::random(), H256::random());
        let sig = key.sign(prehash).unwrap();

        // Set 's' to a high-s value 'SECP256K1::ORDER - s'
        let s = U256::from_be_bytes(sig.s().raw()).neg_mod(&Secp256k1::ORDER);
        let s_high = B256::from(s.to_be_bytes());
        let v_flipped = B8::from(u8::from(sig.v()) ^ 1);
        let high_s_sig = Signature::new(sig.r(), s_high, v_flipped);

        // Normalize the signature
        let normalized = high_s_sig.normalize_s();
        assert_eq!(normalized.r(), sig.r());
        assert_eq!(normalized.s(), sig.s());
        assert_eq!(normalized.v(), sig.v());

        // Validate the signature verification behavior
        key.public().verify(prehash, sig).unwrap();
        key.public().verify(prehash, normalized).unwrap();
        assert!(key.public().verify(prehash, high_s_sig).is_err());
    }

    #[test]
    fn normalize_s_order() {
        use k256::Curve;
        use k256::Encoding;
        use k256::Secp256k1;

        let (key, prehash) = (PrivateKey::random(), H256::random());
        let sig = key.sign(prehash).unwrap();

        // Set 's' to 'SECP256K1::ORDER'
        let s_order = B256::from(Secp256k1::ORDER.to_be_bytes());
        let order_s_sig = Signature::new(sig.r(), s_order, sig.v());

        // Normalize the signature
        let normalized = order_s_sig.normalize_s();
        assert_eq!(normalized.r(), sig.r());
        assert_eq!(normalized.s(), B256::zero());
        assert_eq!(normalized.v(), B8::from(u8::from(sig.v()) ^ 1));

        // Validate the signature verification behavior
        key.public().verify(prehash, sig).unwrap();
        assert!(key.public().verify(prehash, normalized).is_err());
        assert!(key.public().verify(prehash, order_s_sig).is_err());
    }

    #[test]
    fn normalize_s_half_order() {
        use k256::Curve;
        use k256::Encoding;
        use k256::Secp256k1;

        let (key, prehash) = (PrivateKey::random(), H256::random());
        let sig = key.sign(prehash).unwrap();

        // Set 's' to 'SECP256K1::ORDER / 2'
        let s_half = B256::from((Secp256k1::ORDER >> 1).to_be_bytes());
        let half_s_sig = Signature::new(sig.r(), s_half, sig.v());

        // Normalize the signature
        let normalized = half_s_sig.normalize_s();
        assert_eq!(normalized.r(), sig.r());
        assert_eq!(normalized.s(), s_half);
        assert_eq!(normalized.v(), sig.v());

        // Validate the signature verification behavior
        key.public().verify(prehash, sig).unwrap();
        assert!(key.public().verify(prehash, normalized).is_err());
        assert!(key.public().verify(prehash, half_s_sig).is_err());
    }

    #[test]
    fn normalize_s_zero() {
        let (key, prehash) = (PrivateKey::random(), H256::random());
        let sig = key.sign(prehash).unwrap();

        // Set 's' to zero
        let zero_s_sig = Signature::new(sig.r(), B256::zero(), sig.v());

        // Normalize the signature
        let normalized = zero_s_sig.normalize_s();
        assert_eq!(normalized.r(), sig.r());
        assert_eq!(normalized.s(), B256::zero());
        assert_eq!(normalized.v(), sig.v());

        // Validate the signature verification behavior
        key.public().verify(prehash, sig).unwrap();
        assert!(key.public().verify(prehash, normalized).is_err());
        assert!(key.public().verify(prehash, zero_s_sig).is_err());
    }

    #[test]
    fn fmt_display() {
        let str = "0x00c31cb98a95e62fd2e6656c0ca9365e672795056b9b8f79e76a600ca25b0c29f85ec86b132c05a4110fd7b30f414f96ac3be72fc3181334729da0632eb99341a2";
        let sig = Signature::from_hex(str).unwrap();
        let fmt = "0x00c31cb98a95e62fd2e6656c0ca9365e672795056b9b8f79e76a600ca25b0c29f85ec86b132c05a4110fd7b30f414f96ac3be72fc3181334729da0632eb99341a2";
        assert_eq!(f!("{sig}"), fmt);
    }

    #[test]
    fn fmt_debug() {
        let str = "0x00c31cb98a95e62fd2e6656c0ca9365e672795056b9b8f79e76a600ca25b0c29f85ec86b132c05a4110fd7b30f414f96ac3be72fc3181334729da0632eb99341a2";
        let sig = Signature::from_hex(str).unwrap();
        let fmt = "Signature { r: 0xc31cb98a95e62fd2e6656c0ca9365e672795056b9b8f79e76a600ca25b0c29f8, s: 0x5ec86b132c05a4110fd7b30f414f96ac3be72fc3181334729da0632eb99341a2, v: 0x00 }";
        assert_eq!(f!("{sig:?}"), fmt);
    }

    #[test]
    fn fmt_hex_lowercase() {
        let str = "0x00c31cb98a95e62fd2e6656c0ca9365e672795056b9b8f79e76a600ca25b0c29f85ec86b132c05a4110fd7b30f414f96ac3be72fc3181334729da0632eb99341a2";
        let sig = Signature::from_hex(str).unwrap();
        let fmt = "0x00c31cb98a95e62fd2e6656c0ca9365e672795056b9b8f79e76a600ca25b0c29f85ec86b132c05a4110fd7b30f414f96ac3be72fc3181334729da0632eb99341a2";
        assert_eq!(f!("{sig:x}"), fmt);
    }

    #[test]
    fn fmt_hex_uppercase() {
        let str = "0x00c31cb98a95e62fd2e6656c0ca9365e672795056b9b8f79e76a600ca25b0c29f85ec86b132c05a4110fd7b30f414f96ac3be72fc3181334729da0632eb99341a2";
        let sig = Signature::from_hex(str).unwrap();
        let fmt = "0x00C31CB98A95E62FD2E6656C0CA9365E672795056B9B8F79E76A600CA25B0C29F85EC86B132C05A4110FD7B30F414F96AC3BE72FC3181334729DA0632EB99341A2";
        assert_eq!(f!("{sig:X}"), fmt);
    }

    #[test]
    fn from_str() {
        let str = "0x00c31cb98a95e62fd2e6656c0ca9365e672795056b9b8f79e76a600ca25b0c29f85ec86b132c05a4110fd7b30f414f96ac3be72fc3181334729da0632eb99341a2";
        let sig = Signature::from_str(str).unwrap();
        let bytes = FixedBytes::<65>::from_str(str).unwrap();
        assert_eq!(sig.to_bytes(), bytes);
    }

    #[test]
    fn from_hex_lowercase() {
        let str = "0x00c31cb98a95e62fd2e6656c0ca9365e672795056b9b8f79e76a600ca25b0c29f85ec86b132c05a4110fd7b30f414f96ac3be72fc3181334729da0632eb99341a2";
        let sig = Signature::from_hex(str).unwrap();
        let bytes = FixedBytes::<65>::from_hex(str).unwrap();
        assert_eq!(sig.to_bytes(), bytes);
    }

    #[test]
    fn from_hex_uppercase() {
        let str = "0x00c31cb98a95e62fd2e6656c0ca9365e672795056b9b8f79e76a600ca25b0c29f85ec86b132c05a4110fd7b30f414f96ac3be72fc3181334729da0632eb99341a2";
        let sig = Signature::from_hex(str).unwrap();
        let bytes = FixedBytes::<65>::from_hex(str).unwrap();
        assert_eq!(sig.to_bytes(), bytes);
    }

    #[test]
    fn serde_json() {
        let str = "0x00c31cb98a95e62fd2e6656c0ca9365e672795056b9b8f79e76a600ca25b0c29f85ec86b132c05a4110fd7b30f414f96ac3be72fc3181334729da0632eb99341a2";
        let sig = Signature::from_hex(str).unwrap();
        let json = serde::to_string(&sig).unwrap();
        assert_eq!(json, "\"0x00c31cb98a95e62fd2e6656c0ca9365e672795056b9b8f79e76a600ca25b0c29f85ec86b132c05a4110fd7b30f414f96ac3be72fc3181334729da0632eb99341a2\"");
        let de: Signature = serde::from_str(&json).unwrap();
        assert_eq!(de, sig);
    }

    #[test]
    fn serde_value() {
        let str = "0x00c31cb98a95e62fd2e6656c0ca9365e672795056b9b8f79e76a600ca25b0c29f85ec86b132c05a4110fd7b30f414f96ac3be72fc3181334729da0632eb99341a2";
        let sig = Signature::from_hex(str).unwrap();
        let ser = serde::to_value(&sig).unwrap();
        assert_eq!(ser, json! { str });
        let de: Signature = serde::from_value(ser).unwrap();
        assert_eq!(de, sig);
    }
}
