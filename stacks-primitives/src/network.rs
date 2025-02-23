// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

/// A trait representing a Stacks network.
pub trait Network {
    /// The network's unique Chain ID.
    const CHAIN_ID: u32;
    /// The tag byte for transactions.
    const TX_ID: u8;
    /// The tag byte for Pay-to-Public-Key-Hash (P2PKH) addresses.
    const P2PKH: u8;
    /// The tag byte for Pay-to-Script-Hash (P2SH) addresses.
    const P2SH: u8;
}

/// The Stacks mainnet network.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mainnet;

impl Network for Mainnet {
    const CHAIN_ID: u32 = 0x0000_0001;
    const TX_ID: u8 = 0x00;
    const P2PKH: u8 = 0x16;
    const P2SH: u8 = 0x14;
}

/// The Stacks testnet network.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Testnet;

impl Network for Testnet {
    const CHAIN_ID: u32 = 0x8000_0000;
    const TX_ID: u8 = 0x80;
    const P2PKH: u8 = 0x1A;
    const P2SH: u8 = 0x15;
}

/// The Stacks devnet network.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Devnet;

impl Network for Devnet {
    const CHAIN_ID: u32 = 0x8000_0000;
    const TX_ID: u8 = 0x80;
    const P2PKH: u8 = 0x1A;
    const P2SH: u8 = 0x15;
}
