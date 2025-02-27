// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use std::str::FromStr;

use format as f;
use serde_json as serde;
use stacks_primitives::address::Address;
use stacks_primitives::ecdsa::PublicKey;
use stacks_primitives::hex::FromHex;
use stacks_primitives::network::Mainnet;
use stacks_primitives::network::Testnet;

mod burn {
    use super::*;

    #[test]
    fn mainnet() {
        let addr = Address::<Mainnet>::BURN;
        assert_eq!(f!("{addr}"), "SP000000000000000000002Q6VF78");
    }

    #[test]
    fn testnet() {
        let addr = Address::<Testnet>::BURN;
        assert_eq!(f!("{addr}"), "ST000000000000000000002AMW42H");
    }

    #[test]
    fn from_str_mainnet() {
        let str = "SP000000000000000000002Q6VF78";
        let addr = Address::<Mainnet>::from_str(str).unwrap();
        assert_eq!(addr, Address::<Mainnet>::BURN);
    }

    #[test]
    fn from_str_testnet() {
        let str = "ST000000000000000000002AMW42H";
        let addr = Address::<Testnet>::from_str(str).unwrap();
        assert_eq!(addr, Address::<Testnet>::BURN);
    }
}

mod p2pkh {
    use super::*;

    #[test]
    fn mainnet() {
        let str = "0x029eabdfa0902bb7fd449a9c244fea5920986c0cb3f6bddf5a04c15ca60d1df255";
        let key = PublicKey::from_hex(str).unwrap();
        let addr = Address::<Mainnet>::p2pkh(key.to_bytes(true));
        assert_eq!(f!("{addr}"), "SP2SJBWG7W60FR02BP8QQPQ2B781V5CGPKKYTYF4Q");
    }

    #[test]
    fn testnet() {
        let str = "0x029eabdfa0902bb7fd449a9c244fea5920986c0cb3f6bddf5a04c15ca60d1df255";
        let key = PublicKey::from_hex(str).unwrap();
        let addr = Address::<Testnet>::p2pkh(key.to_bytes(true));
        assert_eq!(f!("{addr}"), "ST2SJBWG7W60FR02BP8QQPQ2B781V5CGPKG95CYAN");
    }
}

mod p2wpkh {
    use super::*;
}

mod p2sh {
    use super::*;
}

mod p2wsh {
    use super::*;
}
