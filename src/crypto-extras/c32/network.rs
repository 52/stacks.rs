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
