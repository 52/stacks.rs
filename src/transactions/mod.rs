#[repr(u8)]
pub(crate) enum TransactionVersion {
    Mainnet = 0x00,
    Testnet = 0x80,
}

pub(crate) enum TransactionChainId {
    Testnet = 0x80000000,
    Mainnet = 0x00000001,
}

#[repr(u8)]
pub(crate) enum TransactionAnchorMode {
    OnChainOnly = 0x01,
    OffChainOnly = 0x02,
    Any = 0x03,
}

#[repr(u8)]
pub(crate) enum TransactionAuthType {
    Standard = 0x04,
    Sponsored = 0x05,
}

#[repr(u8)]
pub(crate) enum TransactionPostConditionMode {
    Allow = 0x01,
    Deny = 0x02,
}

#[repr(u8)]
pub(crate) enum TransactionPostConditionType {
    STX = 0x00,
    Fungible = 0x01,
    NonFungible = 0x02,
}

pub(crate) struct StacksTransaction {
    pub version: TransactionVersion,
    pub chain_id: TransactionChainId,
    pub auth: TransactionAuthType,
    pub anchor_mode: TransactionAnchorMode,
    pub post_condition_mode: TransactionPostConditionMode,
    // post_conditions
    // payload
}
