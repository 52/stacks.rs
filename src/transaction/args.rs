use crate::clarity::ClarityValue;
use crate::transaction::AnchorMode;
use crate::transaction::Error;
use crate::transaction::PostConditionMode;
use crate::transaction::PostConditions;
use crate::StacksNetwork;
use crate::StacksPrivateKey;
use crate::StacksPublicKey;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct USTXTokenTransferOptions {
    pub recipient: ClarityValue,
    pub public_key: StacksPublicKey,
    pub amount: u64,
    pub fee: u64,
    pub nonce: u64,
    pub network: StacksNetwork,
    pub anchor_mode: AnchorMode,
    pub memo: String,
    pub post_condition_mode: PostConditionMode,
    pub post_conditions: PostConditions,
    pub sponsored: bool,
}

impl USTXTokenTransferOptions {
    pub fn new(
        recipient: ClarityValue,
        public_key: StacksPublicKey,
        amount: u64,
        fee: u64,
        nonce: u64,
        network: StacksNetwork,
        anchor_mode: AnchorMode,
        memo: impl Into<String>,
        post_condition_mode: PostConditionMode,
        post_conditions: PostConditions,
        sponsored: bool,
    ) -> Self {
        Self {
            recipient,
            public_key,
            amount,
            fee,
            nonce,
            network,
            anchor_mode,
            memo: memo.into(),
            post_condition_mode,
            post_conditions,
            sponsored,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct STXTokenTransferOptions {
    pub recipient: ClarityValue,
    pub sender_key: StacksPrivateKey,
    pub amount: u64,
    pub fee: u64,
    pub nonce: u64,
    pub network: StacksNetwork,
    pub anchor_mode: AnchorMode,
    pub memo: String,
    pub post_condition_mode: PostConditionMode,
    pub post_conditions: PostConditions,
    pub sponsored: bool,
}

impl TryFrom<STXTokenTransferOptions> for USTXTokenTransferOptions {
    type Error = Error;

    fn try_from(args: STXTokenTransferOptions) -> Result<Self, Self::Error> {
        let public_key = args.sender_key.public_key(&secp256k1::Secp256k1::new());

        Ok(Self::new(
            args.recipient,
            public_key,
            args.amount,
            args.fee,
            args.nonce,
            args.network,
            args.anchor_mode,
            args.memo,
            args.post_condition_mode,
            args.post_conditions,
            args.sponsored,
        ))
    }
}

impl STXTokenTransferOptions {
    pub fn new(
        recipient: ClarityValue,
        sender_key: StacksPrivateKey,
        amount: u64,
        fee: u64,
        nonce: u64,
        network: StacksNetwork,
        anchor_mode: AnchorMode,
        memo: impl Into<String>,
        post_condition_mode: PostConditionMode,
        post_conditions: PostConditions,
        sponsored: bool,
    ) -> Self {
        Self {
            recipient,
            sender_key,
            amount,
            fee,
            nonce,
            network,
            anchor_mode,
            memo: memo.into(),
            post_condition_mode,
            post_conditions,
            sponsored,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct USTXTokenTransferOptionsMSig {
    pub recipient: ClarityValue,
    pub public_keys: Vec<StacksPublicKey>,
    pub signatures: u8,
    pub amount: u64,
    pub fee: u64,
    pub nonce: u64,
    pub network: StacksNetwork,
    pub anchor_mode: AnchorMode,
    pub memo: String,
    pub post_condition_mode: PostConditionMode,
    pub post_conditions: PostConditions,
    pub sponsored: bool,
}

impl USTXTokenTransferOptionsMSig {
    pub fn new(
        recipient: ClarityValue,
        public_keys: Vec<StacksPublicKey>,
        signatures: u8,
        amount: u64,
        fee: u64,
        nonce: u64,
        network: StacksNetwork,
        anchor_mode: AnchorMode,
        memo: impl Into<String>,
        post_condition_mode: PostConditionMode,
        post_conditions: PostConditions,
        sponsored: bool,
    ) -> Self {
        Self {
            recipient,
            public_keys,
            signatures,
            amount,
            fee,
            nonce,
            network,
            anchor_mode,
            memo: memo.into(),
            post_condition_mode,
            post_conditions,
            sponsored,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct STXTokenTransferOptionsMSig {
    pub recipient: ClarityValue,
    pub signer_keys: Vec<StacksPrivateKey>,
    pub public_keys: Vec<StacksPublicKey>,
    pub signatures: u8,
    pub amount: u64,
    pub fee: u64,
    pub nonce: u64,
    pub network: StacksNetwork,
    pub anchor_mode: AnchorMode,
    pub memo: String,
    pub post_condition_mode: PostConditionMode,
    pub post_conditions: PostConditions,
    pub sponsored: bool,
}

impl From<STXTokenTransferOptionsMSig> for USTXTokenTransferOptionsMSig {
    fn from(args: STXTokenTransferOptionsMSig) -> Self {
        Self::new(
            args.recipient,
            args.public_keys,
            args.signatures,
            args.amount,
            args.fee,
            args.nonce,
            args.network,
            args.anchor_mode,
            args.memo,
            args.post_condition_mode,
            args.post_conditions,
            args.sponsored,
        )
    }
}

impl STXTokenTransferOptionsMSig {
    pub fn new(
        recipient: ClarityValue,
        signer_keys: Vec<StacksPrivateKey>,
        public_keys: Vec<StacksPublicKey>,
        signatures: u8,
        amount: u64,
        fee: u64,
        nonce: u64,
        network: StacksNetwork,
        anchor_mode: AnchorMode,
        memo: impl Into<String>,
        post_condition_mode: PostConditionMode,
        post_conditions: PostConditions,
        sponsored: bool,
    ) -> Self {
        Self {
            recipient,
            signer_keys,
            public_keys,
            signatures,
            amount,
            fee,
            nonce,
            network,
            anchor_mode,
            memo: memo.into(),
            post_condition_mode,
            post_conditions,
            sponsored,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UContractCallOptions {
    pub contract: ClarityValue,
    pub function_name: String,
    pub function_args: Vec<ClarityValue>,
    pub public_key: StacksPublicKey,
    pub fee: u64,
    pub nonce: u64,
    pub network: StacksNetwork,
    pub anchor_mode: AnchorMode,
    pub post_condition_mode: PostConditionMode,
    pub post_conditions: PostConditions,
    pub sponsored: bool,
}

impl UContractCallOptions {
    pub fn new(
        contract: ClarityValue,
        function_name: impl Into<String>,
        function_args: impl Into<Vec<ClarityValue>>,
        public_key: StacksPublicKey,
        fee: u64,
        nonce: u64,
        network: StacksNetwork,
        anchor_mode: AnchorMode,
        post_condition_mode: PostConditionMode,
        post_conditions: PostConditions,
        sponsored: bool,
    ) -> Self {
        Self {
            contract,
            function_name: function_name.into(),
            function_args: function_args.into(),
            public_key,
            fee,
            nonce,
            network,
            anchor_mode,
            post_condition_mode,
            post_conditions,
            sponsored,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractCallOptions {
    pub contract: ClarityValue,
    pub function_name: String,
    pub function_args: Vec<ClarityValue>,
    pub sender_key: StacksPrivateKey,
    pub fee: u64,
    pub nonce: u64,
    pub network: StacksNetwork,
    pub anchor_mode: AnchorMode,
    pub post_condition_mode: PostConditionMode,
    pub post_conditions: PostConditions,
    pub sponsored: bool,
}

impl From<ContractCallOptions> for UContractCallOptions {
    fn from(args: ContractCallOptions) -> Self {
        let public_key = args.sender_key.public_key(&secp256k1::Secp256k1::new());

        Self::new(
            args.contract,
            args.function_name,
            args.function_args,
            public_key,
            args.fee,
            args.nonce,
            args.network,
            args.anchor_mode,
            args.post_condition_mode,
            args.post_conditions,
            args.sponsored,
        )
    }
}

impl ContractCallOptions {
    pub fn new(
        contract: ClarityValue,
        function_name: impl Into<String>,
        function_args: impl Into<Vec<ClarityValue>>,
        sender_key: StacksPrivateKey,
        fee: u64,
        nonce: u64,
        network: StacksNetwork,
        anchor_mode: AnchorMode,
        post_condition_mode: PostConditionMode,
        post_conditions: PostConditions,
        sponsored: bool,
    ) -> Self {
        Self {
            contract,
            function_name: function_name.into(),
            function_args: function_args.into(),
            sender_key,
            fee,
            nonce,
            network,
            anchor_mode,
            post_condition_mode,
            post_conditions,
            sponsored,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UContractCallOptionsMSig {
    pub contract: ClarityValue,
    pub function_name: String,
    pub function_args: Vec<ClarityValue>,
    pub public_keys: Vec<StacksPublicKey>,
    pub signatures: u8,
    pub fee: u64,
    pub nonce: u64,
    pub network: StacksNetwork,
    pub anchor_mode: AnchorMode,
    pub post_condition_mode: PostConditionMode,
    pub post_conditions: PostConditions,
    pub sponsored: bool,
}

impl UContractCallOptionsMSig {
    pub fn new(
        contract: ClarityValue,
        function_name: impl Into<String>,
        function_args: impl Into<Vec<ClarityValue>>,
        public_keys: Vec<StacksPublicKey>,
        signatures: u8,
        fee: u64,
        nonce: u64,
        network: StacksNetwork,
        anchor_mode: AnchorMode,
        post_condition_mode: PostConditionMode,
        post_conditions: PostConditions,
        sponsored: bool,
    ) -> Self {
        Self {
            contract,
            function_name: function_name.into(),
            function_args: function_args.into(),
            public_keys,
            signatures,
            fee,
            nonce,
            network,
            anchor_mode,
            post_condition_mode,
            post_conditions,
            sponsored,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractCallOptionsMSig {
    pub contract: ClarityValue,
    pub function_name: String,
    pub function_args: Vec<ClarityValue>,
    pub signer_keys: Vec<StacksPrivateKey>,
    pub public_keys: Vec<StacksPublicKey>,
    pub signatures: u8,
    pub fee: u64,
    pub nonce: u64,
    pub network: StacksNetwork,
    pub anchor_mode: AnchorMode,
    pub post_condition_mode: PostConditionMode,
    pub post_conditions: PostConditions,
    pub sponsored: bool,
}

impl From<ContractCallOptionsMSig> for UContractCallOptionsMSig {
    fn from(args: ContractCallOptionsMSig) -> Self {
        Self::new(
            args.contract,
            args.function_name,
            args.function_args,
            args.public_keys,
            args.signatures,
            args.fee,
            args.nonce,
            args.network,
            args.anchor_mode,
            args.post_condition_mode,
            args.post_conditions,
            args.sponsored,
        )
    }
}

impl ContractCallOptionsMSig {
    pub fn new(
        contract: ClarityValue,
        function_name: impl Into<String>,
        function_args: impl Into<Vec<ClarityValue>>,
        signer_keys: Vec<StacksPrivateKey>,
        public_keys: Vec<StacksPublicKey>,
        signatures: u8,
        fee: u64,
        nonce: u64,
        network: StacksNetwork,
        anchor_mode: AnchorMode,
        post_condition_mode: PostConditionMode,
        post_conditions: PostConditions,
        sponsored: bool,
    ) -> Self {
        Self {
            contract,
            function_name: function_name.into(),
            function_args: function_args.into(),
            signer_keys,
            public_keys,
            signatures,
            fee,
            nonce,
            network,
            anchor_mode,
            post_condition_mode,
            post_conditions,
            sponsored,
        }
    }
}
