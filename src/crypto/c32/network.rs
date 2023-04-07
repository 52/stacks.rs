use crate::crypto::base58::network::BitcoinNetworkVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StacksNetworkVersion {
    MainnetP2PKH = 22,
    MainnetP2SH = 20,
    TestnetP2PKH = 26,
    TestnetP2SH = 21,
}

impl StacksNetworkVersion {
    pub(crate) fn version(&self) -> u8 {
        match self {
            StacksNetworkVersion::MainnetP2PKH => 22,
            StacksNetworkVersion::MainnetP2SH => 20,
            StacksNetworkVersion::TestnetP2PKH => 26,
            StacksNetworkVersion::TestnetP2SH => 21,
        }
    }

    pub(crate) fn to_bitcoin_network_version(self) -> BitcoinNetworkVersion {
        match self {
            StacksNetworkVersion::MainnetP2PKH => BitcoinNetworkVersion::MainnetP2PKH,
            StacksNetworkVersion::MainnetP2SH => BitcoinNetworkVersion::MainnetP2SH,
            StacksNetworkVersion::TestnetP2PKH => BitcoinNetworkVersion::TestnetP2PKH,
            StacksNetworkVersion::TestnetP2SH => BitcoinNetworkVersion::TestnetP2SH,
        }
    }
}

impl From<u8> for StacksNetworkVersion {
    fn from(value: u8) -> Self {
        match value {
            22 => StacksNetworkVersion::MainnetP2PKH,
            20 => StacksNetworkVersion::MainnetP2SH,
            26 => StacksNetworkVersion::TestnetP2PKH,
            21 => StacksNetworkVersion::TestnetP2SH,
            _ => panic!("Invalid network version"),
        }
    }
}
