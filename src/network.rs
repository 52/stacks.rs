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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StacksNetwork {
    version: TransactionVersion,
    chain_id: ChainID,
}

impl From<TransactionVersion> for StacksNetwork {
    fn from(version: TransactionVersion) -> Self {
        match version {
            TransactionVersion::Mainnet => Self::mainnet(),
            TransactionVersion::Testnet => Self::testnet(),
        }
    }
}

impl StacksNetwork {
    /// Creates a new `StacksNetwork`.
    pub fn new(chain_id: ChainID, version: TransactionVersion) -> Self {
        Self { version, chain_id }
    }

    /// Returns the mainnet network.
    pub fn mainnet() -> Self {
        Self::new(ChainID::Mainnet, TransactionVersion::Mainnet)
    }

    /// Returns the testnet network.
    pub fn testnet() -> Self {
        Self::new(ChainID::Testnet, TransactionVersion::Testnet)
    }

    /// Returns the transaction version.
    pub fn version(self) -> TransactionVersion {
        self.version
    }

    /// Returns the chain ID.
    pub fn chain_id(self) -> ChainID {
        self.chain_id
    }
}
