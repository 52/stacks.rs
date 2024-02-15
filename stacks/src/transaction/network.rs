// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

/// The mainnet Hiro API URL.
pub const HIRO_MAINNET_DEFAULT: &str = "https://api.mainnet.hiro.so";
/// The testnet Hiro API URL.
pub const HIRO_TESTNET_DEFAULT: &str = "https://api.testnet.hiro.so";
/// The mocknet API URL.
pub const HIRO_MOCKNET_DEFAULT: &str = "http://localhost:3999";

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransactionVersion {
    Mainnet = 0x00,
    Testnet = 0x80,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChainID {
    Testnet = 0x8000_0000,
    Mainnet = 0x0000_0001,
}

pub trait Network {
    /// Returns the transaction version.
    fn version(&self) -> TransactionVersion;
    /// Returns the chain ID.
    fn chain_id(&self) -> ChainID;
    /// Returns the API base URL.
    fn base_url(&self) -> String;
}

impl_network_type!(
    StacksMainnet,
    TransactionVersion::Mainnet,
    ChainID::Mainnet,
    HIRO_MAINNET_DEFAULT
);

impl_network_type!(
    StacksTestnet,
    TransactionVersion::Testnet,
    ChainID::Testnet,
    HIRO_TESTNET_DEFAULT
);

impl_network_type!(
    StacksMocknet,
    TransactionVersion::Testnet,
    ChainID::Testnet,
    HIRO_MOCKNET_DEFAULT
);

macro_rules! impl_network_type {
    ($name:ident, $version:expr, $chain_id:expr, $api:expr) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name {
            version: TransactionVersion,
            chain_id: ChainID,
        }
        impl $name {
            pub fn new() -> Self {
                Self {
                    version: $version,
                    chain_id: $chain_id,
                }
            }
        }
        impl Network for $name {
            fn version(&self) -> TransactionVersion {
                self.version
            }

            fn chain_id(&self) -> ChainID {
                self.chain_id
            }

            fn base_url(&self) -> String {
                $api.into()
            }
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
        impl AsRef<$name> for $name {
            fn as_ref(&self) -> &Self {
                self
            }
        }
    };
}

pub(crate) use impl_network_type;
