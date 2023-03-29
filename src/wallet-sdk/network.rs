#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitcoinNetworkVersion {
    MainnetP2PKH = 0,
    MainnetP2SH = 5,
    TestnetP2PKH = 111,
    TestnetP2SH = 196,
}

impl BitcoinNetworkVersion {
    pub(crate) fn as_ref(&self) -> u8 {
        match self {
            BitcoinNetworkVersion::MainnetP2PKH => 0,
            BitcoinNetworkVersion::MainnetP2SH => 5,
            BitcoinNetworkVersion::TestnetP2PKH => 111,
            BitcoinNetworkVersion::TestnetP2SH => 196,
        }
    }

    pub(crate) fn to_stacks_network(&self) -> StacksNetworkVersion {
        match self {
            BitcoinNetworkVersion::MainnetP2PKH => StacksNetworkVersion::MainnetP2PKH,
            BitcoinNetworkVersion::MainnetP2SH => StacksNetworkVersion::MainnetP2SH,
            BitcoinNetworkVersion::TestnetP2PKH => StacksNetworkVersion::TestnetP2PKH,
            BitcoinNetworkVersion::TestnetP2SH => StacksNetworkVersion::TestnetP2SH,
        }
    }
}

impl From<BitcoinNetworkVersion> for u8 {
    fn from(version: BitcoinNetworkVersion) -> Self {
        match version {
            BitcoinNetworkVersion::MainnetP2PKH => 0,
            BitcoinNetworkVersion::MainnetP2SH => 5,
            BitcoinNetworkVersion::TestnetP2PKH => 111,
            BitcoinNetworkVersion::TestnetP2SH => 196,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StacksNetworkVersion {
    MainnetP2PKH = 22,
    MainnetP2SH = 20,
    TestnetP2PKH = 26,
    TestnetP2SH = 21,
}

impl StacksNetworkVersion {
    pub(crate) fn as_ref(&self) -> u8 {
        match self {
            StacksNetworkVersion::MainnetP2PKH => 22,
            StacksNetworkVersion::MainnetP2SH => 20,
            StacksNetworkVersion::TestnetP2PKH => 26,
            StacksNetworkVersion::TestnetP2SH => 21,
        }
    }

    pub(crate) fn to_bitcoin_version(&self) -> BitcoinNetworkVersion {
        match self {
            StacksNetworkVersion::MainnetP2PKH => BitcoinNetworkVersion::MainnetP2PKH,
            StacksNetworkVersion::MainnetP2SH => BitcoinNetworkVersion::MainnetP2SH,
            StacksNetworkVersion::TestnetP2PKH => BitcoinNetworkVersion::TestnetP2PKH,
            StacksNetworkVersion::TestnetP2SH => BitcoinNetworkVersion::TestnetP2SH,
        }
    }
}

impl From<StacksNetworkVersion> for u8 {
    fn from(version: StacksNetworkVersion) -> Self {
        match version {
            StacksNetworkVersion::MainnetP2PKH => 22,
            StacksNetworkVersion::MainnetP2SH => 20,
            StacksNetworkVersion::TestnetP2PKH => 26,
            StacksNetworkVersion::TestnetP2SH => 21,
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
