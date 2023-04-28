pub const HIRO_MAINNET_DEFAULT: &str = "https://stacks-node-api.mainnet.stacks.co";
pub const HIRO_TESTNET_DEFAULT: &str = "https://stacks-node-api.testnet.stacks.co";
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

macro_rules! impl_default_network {
    ($type:ty, $version:expr, $chain_id:expr) => {
        impl Default for $type {
            fn default() -> Self {
                Self::new()
            }
        }

        impl AsRef<$type> for $type {
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl $type {
            pub fn new() -> Self {
                Self {
                    version: $version,
                    chain_id: $chain_id,
                }
            }
        }
    };
}

pub trait Network: std::fmt::Debug + Clone {
    /// Returns the transaction version.
    fn version(&self) -> TransactionVersion;
    /// Returns the chain ID.
    fn chain_id(&self) -> ChainID;
    /// Returns the API base URL.
    fn base_url(&self) -> String;
}

/// The mainnet network.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StacksMainnet {
    version: TransactionVersion,
    chain_id: ChainID,
}

impl_default_network!(StacksMainnet, TransactionVersion::Mainnet, ChainID::Mainnet);
impl Network for StacksMainnet {
    fn version(&self) -> TransactionVersion {
        self.version
    }

    fn chain_id(&self) -> ChainID {
        self.chain_id
    }

    fn base_url(&self) -> String {
        HIRO_MAINNET_DEFAULT.into()
    }
}

/// The testnet network.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StacksTestnet {
    version: TransactionVersion,
    chain_id: ChainID,
}

impl_default_network!(StacksTestnet, TransactionVersion::Testnet, ChainID::Testnet);
impl Network for StacksTestnet {
    fn version(&self) -> TransactionVersion {
        self.version
    }

    fn chain_id(&self) -> ChainID {
        self.chain_id
    }

    fn base_url(&self) -> String {
        HIRO_TESTNET_DEFAULT.into()
    }
}

/// The mocknet network.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StacksMocknet {
    version: TransactionVersion,
    chain_id: ChainID,
}

impl_default_network!(StacksMocknet, TransactionVersion::Testnet, ChainID::Testnet);
impl Network for StacksMocknet {
    fn version(&self) -> TransactionVersion {
        self.version
    }

    fn chain_id(&self) -> ChainID {
        self.chain_id
    }

    fn base_url(&self) -> String {
        HIRO_MOCKNET_DEFAULT.into()
    }
}
