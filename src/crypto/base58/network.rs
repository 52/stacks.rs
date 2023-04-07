use crate::crypto::c32::network::StacksNetworkVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitcoinNetworkVersion {
    MainnetP2PKH = 0,
    MainnetP2SH = 5,
    TestnetP2PKH = 111,
    TestnetP2SH = 196,
}

impl BitcoinNetworkVersion {
    pub(crate) fn version(self) -> u8 {
        match self {
            BitcoinNetworkVersion::MainnetP2PKH => 0,
            BitcoinNetworkVersion::MainnetP2SH => 5,
            BitcoinNetworkVersion::TestnetP2PKH => 111,
            BitcoinNetworkVersion::TestnetP2SH => 196,
        }
    }

    pub(crate) fn to_stacks_network_version(self) -> StacksNetworkVersion {
        match self {
            BitcoinNetworkVersion::MainnetP2PKH => StacksNetworkVersion::MainnetP2PKH,
            BitcoinNetworkVersion::MainnetP2SH => StacksNetworkVersion::MainnetP2SH,
            BitcoinNetworkVersion::TestnetP2PKH => StacksNetworkVersion::TestnetP2PKH,
            BitcoinNetworkVersion::TestnetP2SH => StacksNetworkVersion::TestnetP2SH,
        }
    }
}

impl From<u8> for BitcoinNetworkVersion {
    fn from(value: u8) -> Self {
        match value {
            0 => BitcoinNetworkVersion::MainnetP2PKH,
            5 => BitcoinNetworkVersion::MainnetP2SH,
            111 => BitcoinNetworkVersion::TestnetP2PKH,
            196 => BitcoinNetworkVersion::TestnetP2SH,
            _ => panic!("Invalid network version"),
        }
    }
}
