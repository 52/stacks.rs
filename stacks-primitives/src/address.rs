// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

/// Hashing scheme for addresses and signatures.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    /// Legacy single-signature address (hash of a single pubkey)
    P2PKH = 0x00,
    /// Legacy multi-signature address (sequential signing)
    P2SH = 0x01,
    /// Segwit single-signature address
    P2WPKH = 0x02,
    /// Segwit multi-signature address (sequential signing)
    P2WSH = 0x03,
    /// Multi-signature without sequential signing requirement
    P2SHNonSequential = 0x05,
    /// Segwit multi-signature without sequential signing requirement
    P2WSHNonSequential = 0x07,
}

/// Network and address type identifier for Stacks addresses.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Version {
    /// Standard single-signature address on mainnet
    MainnetP2PKH = 22,
    /// Standard single-signature address on testnet
    TestnetP2PKH = 26,
    /// Multi-signature address on mainnet
    MainnetP2SH = 20,
    /// Multi-signature address on testnet
    TestnetP2SH = 21,
}

/// Representation of a Stacks address.
pub struct Address {
    /// Network version (mainnet/testnet) and type (P2PKH/P2SH)
    pub version: Version,
    /// Hash160 of public key(s) or script
    pub hash: [u8; 20],
}
