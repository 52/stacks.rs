use crate::clarity::ClarityValue;
use crate::clarity::LengthPrefixedString;
use crate::clarity::StandardPrincipalCV;
use crate::clarity::CLARITY_TYPE_PRINCIPAL_CONTRACT;
use crate::clarity::CLARITY_TYPE_PRINCIPAL_STANDARD;
use crate::crypto::Serialize;
use crate::transaction::Error;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnchorMode {
    OnChain = 0x01,
    OffChain = 0x02,
    Any = 0x03,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PostConditionMode {
    Allow = 0x01,
    Deny = 0x02,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PostConditionPrincipalID {
    Standard = 0x02,
    Contract = 0x03,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PostConditionType {
    Stx = 0x00,
    Fungible = 0x01,
    NonFungible = 0x02,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FungibleConditionCode {
    Equal = 0x01,
    Greater = 0x02,
    GreaterEqual = 0x03,
    Less = 0x04,
    LessEqual = 0x05,
}

impl TryFrom<u8> for FungibleConditionCode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(FungibleConditionCode::Equal),
            0x02 => Ok(FungibleConditionCode::Greater),
            0x03 => Ok(FungibleConditionCode::GreaterEqual),
            0x04 => Ok(FungibleConditionCode::Less),
            0x05 => Ok(FungibleConditionCode::LessEqual),
            _ => Err(Error::InvalidConditionCode),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NonFungibleConditionCode {
    DoesNotOwn = 0x10,
    Owns = 0x11,
}

impl TryFrom<u8> for NonFungibleConditionCode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x10 => Ok(NonFungibleConditionCode::DoesNotOwn),
            0x11 => Ok(NonFungibleConditionCode::Owns),
            _ => Err(Error::InvalidConditionCode),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssetInfo {
    address: ClarityValue,
    contract_name: LengthPrefixedString,
    asset_name: LengthPrefixedString,
}

impl AssetInfo {
    pub fn new(
        address: impl Into<String>,
        contract_name: impl Into<String>,
        asset_name: impl Into<String>,
    ) -> AssetInfo {
        Self {
            address: StandardPrincipalCV::new(address),
            contract_name: LengthPrefixedString::new(contract_name),
            asset_name: LengthPrefixedString::new(asset_name),
        }
    }
}

impl Serialize for AssetInfo {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![];
        buff.extend_from_slice(&self.address.serialize()?[1..]);
        buff.extend_from_slice(&self.contract_name.serialize()?);
        buff.extend_from_slice(&self.asset_name.serialize()?);
        Ok(buff)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PostCondition {
    Stx(STXPostCondition),
    Fungible(FungiblePostCondition),
    NonFungible(NonFungiblePostCondition),
}

impl Serialize for PostCondition {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        match self {
            PostCondition::Stx(cond) => cond.serialize(),
            PostCondition::Fungible(cond) => cond.serialize(),
            PostCondition::NonFungible(cond) => cond.serialize(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PostConditions(Vec<PostCondition>);

impl PostConditions {
    pub fn new(values: impl Into<Vec<PostCondition>>) -> Self {
        PostConditions(values.into())
    }

    pub fn empty() -> Self {
        PostConditions(vec![])
    }
}

impl PostConditions {
    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut buff = vec![];
        buff.extend_from_slice(&u32::try_from(self.0.len())?.to_be_bytes());

        for value in &self.0 {
            buff.extend_from_slice(&value.serialize()?);
        }

        Ok(buff)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct STXPostCondition {
    principal: ClarityValue,
    amount: u64,
    condition_code: FungibleConditionCode,
}

impl STXPostCondition {
    pub fn new(
        principal: ClarityValue,
        amount: u64,
        condition_code: FungibleConditionCode,
    ) -> PostCondition {
        let condition = Self {
            principal,
            amount,
            condition_code,
        };

        PostCondition::Stx(condition)
    }
}

impl Serialize for STXPostCondition {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![PostConditionType::Stx as u8];
        let type_id = self.principal.type_id();

        match type_id {
            CLARITY_TYPE_PRINCIPAL_STANDARD => {
                buff.push(PostConditionPrincipalID::Standard as u8);
            }
            CLARITY_TYPE_PRINCIPAL_CONTRACT => {
                buff.push(PostConditionPrincipalID::Contract as u8);
            }
            _ => return Err(Error::InvalidPrincipalType(type_id)),
        }

        buff.extend_from_slice(&self.principal.serialize()?[1..]);
        buff.push(self.condition_code as u8);
        buff.extend_from_slice(&self.amount.to_be_bytes());
        Ok(buff)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FungiblePostCondition {
    pub principal: ClarityValue,
    pub asset_info: AssetInfo,
    pub amount: u64,
    pub condition_code: FungibleConditionCode,
}

impl FungiblePostCondition {
    pub fn new(
        principal: ClarityValue,
        asset_info: AssetInfo,
        amount: u64,
        condition_code: FungibleConditionCode,
    ) -> PostCondition {
        let condition = Self {
            principal,
            asset_info,
            amount,
            condition_code,
        };

        PostCondition::Fungible(condition)
    }
}

impl Serialize for FungiblePostCondition {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![PostConditionType::Fungible as u8];
        let type_id = self.principal.type_id();

        match type_id {
            CLARITY_TYPE_PRINCIPAL_STANDARD => {
                buff.push(PostConditionPrincipalID::Standard as u8);
            }
            CLARITY_TYPE_PRINCIPAL_CONTRACT => {
                buff.push(PostConditionPrincipalID::Contract as u8);
            }
            _ => return Err(Error::InvalidPrincipalType(type_id)),
        }

        buff.extend_from_slice(&self.principal.serialize()?[1..]);
        buff.extend_from_slice(&self.asset_info.serialize()?);
        buff.push(self.condition_code as u8);
        buff.extend_from_slice(&self.amount.to_be_bytes());
        Ok(buff)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonFungiblePostCondition {
    pub principal: ClarityValue,
    pub asset_info: AssetInfo,
    pub asset_name: ClarityValue,
    pub condition_code: NonFungibleConditionCode,
}

impl NonFungiblePostCondition {
    pub fn new(
        principal: ClarityValue,
        asset_info: AssetInfo,
        asset_name: ClarityValue,
        condition_code: NonFungibleConditionCode,
    ) -> PostCondition {
        let condition = Self {
            principal,
            asset_info,
            asset_name,
            condition_code,
        };

        PostCondition::NonFungible(condition)
    }
}

impl Serialize for NonFungiblePostCondition {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![PostConditionType::NonFungible as u8];
        let type_id = self.principal.type_id();

        match type_id {
            CLARITY_TYPE_PRINCIPAL_STANDARD => {
                buff.push(PostConditionPrincipalID::Standard as u8);
            }
            CLARITY_TYPE_PRINCIPAL_CONTRACT => {
                buff.push(PostConditionPrincipalID::Contract as u8);
            }
            _ => return Err(Error::InvalidPrincipalType(type_id)),
        }

        buff.extend_from_slice(&self.principal.serialize()?[1..]);
        buff.extend_from_slice(&self.asset_info.serialize()?);
        buff.extend_from_slice(&self.asset_name.serialize()?);
        buff.push(self.condition_code as u8);
        Ok(buff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clarity::ContractPrincipalCV;
    use crate::clarity::StandardPrincipalCV;
    use crate::clarity::UIntCV;
    use crate::crypto::hex::bytes_to_hex;

    #[test]
    fn test_post_conditions() {
        let info = AssetInfo::new(
            "SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B",
            "my-contract",
            "my-asset",
        );

        let list = PostConditions::new([
            STXPostCondition::new(
                StandardPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B"),
                1000000,
                FungibleConditionCode::GreaterEqual,
            ),
            STXPostCondition::new(
                ContractPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B", "test"),
                1000000,
                FungibleConditionCode::GreaterEqual,
            ),
            NonFungiblePostCondition::new(
                StandardPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B"),
                info.clone(),
                UIntCV::new(60149),
                NonFungibleConditionCode::Owns,
            ),
            FungiblePostCondition::new(
                ContractPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B", "test"),
                info.clone(),
                1000000,
                FungibleConditionCode::GreaterEqual,
            ),
        ]);

        let serialized = list.serialize().unwrap();
        let hex = bytes_to_hex(&serialized);

        let expected = "00000004000216a5d9d331000f5b79578ce56bd157f29a9056f0d60300000000000f4240000316a5d9d331000f5b79578ce56bd157f29a9056f0d604746573740300000000000f4240020216a5d9d331000f5b79578ce56bd157f29a9056f0d616a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d6173736574010000000000000000000000000000eaf511010316a5d9d331000f5b79578ce56bd157f29a9056f0d6047465737416a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d61737365740300000000000f4240";
        assert_eq!(hex, expected);
    }

    #[test]
    fn test_asset_info() {
        let info = AssetInfo::new(
            "SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B",
            "my-contract",
            "my-asset",
        );

        let serialized = info.serialize().unwrap();
        let hex = bytes_to_hex(&serialized);

        let expected =
            "16a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d6173736574";
        assert_eq!(hex, expected);
    }

    #[test]
    fn test_stx_post_condition() {
        let s_cv = StandardPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B");
        let s_pc = STXPostCondition::new(s_cv, 1000000, FungibleConditionCode::GreaterEqual);

        let s_pc_ser = s_pc.serialize().unwrap();
        let s_pc_hex = bytes_to_hex(&s_pc_ser);

        let s_pc_expected = "000216a5d9d331000f5b79578ce56bd157f29a9056f0d60300000000000f4240";
        assert_eq!(s_pc_hex, s_pc_expected);

        let c_cv = ContractPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B", "test");
        let c_pc = STXPostCondition::new(c_cv, 1000000, FungibleConditionCode::GreaterEqual);

        let c_pc_ser = c_pc.serialize().unwrap();
        let c_pc_hex = bytes_to_hex(&c_pc_ser);

        let c_pc_expected =
            "000316a5d9d331000f5b79578ce56bd157f29a9056f0d604746573740300000000000f4240";
        assert_eq!(c_pc_hex, c_pc_expected)
    }

    #[test]
    fn test_fungible_post_condition() {
        let s_cv = StandardPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B");
        let info = AssetInfo::new(
            "SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B",
            "my-contract",
            "my-asset",
        );

        let s_pc =
            FungiblePostCondition::new(s_cv, info.clone(), 1000000, FungibleConditionCode::Equal);

        let s_pc_ser = s_pc.serialize().unwrap();
        let s_pc_hex = bytes_to_hex(&s_pc_ser);

        let s_pc_expected = "010216a5d9d331000f5b79578ce56bd157f29a9056f0d616a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d61737365740100000000000f4240";
        assert_eq!(s_pc_hex, s_pc_expected);

        let c_cv = ContractPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B", "test");
        let c_pc = FungiblePostCondition::new(c_cv, info, 1000000, FungibleConditionCode::Equal);

        let c_pc_ser = c_pc.serialize().unwrap();
        let c_pc_hex = bytes_to_hex(&c_pc_ser);

        let c_pc_expected = "010316a5d9d331000f5b79578ce56bd157f29a9056f0d6047465737416a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d61737365740100000000000f4240";
        assert_eq!(c_pc_hex, c_pc_expected)
    }

    #[test]
    fn test_non_fungible_post_condition() {
        let nft_asset = UIntCV::new(60419);

        let s_cv = StandardPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B");
        let info = AssetInfo::new(
            "SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B",
            "my-contract",
            "my-asset",
        );

        let s_pc = NonFungiblePostCondition::new(
            s_cv,
            info.clone(),
            nft_asset.clone(),
            NonFungibleConditionCode::Owns,
        );

        let s_pc_ser = s_pc.serialize().unwrap();
        let s_pc_hex = bytes_to_hex(&s_pc_ser);

        let s_pc_expected = "020216a5d9d331000f5b79578ce56bd157f29a9056f0d616a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d6173736574010000000000000000000000000000ec0311";
        assert_eq!(s_pc_hex, s_pc_expected);

        let c_cv = ContractPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B", "test");
        let c_pc =
            NonFungiblePostCondition::new(c_cv, info, nft_asset, NonFungibleConditionCode::Owns);

        let c_pc_ser = c_pc.serialize().unwrap();
        let c_pc_hex = bytes_to_hex(&c_pc_ser);

        let c_pc_expected = "020316a5d9d331000f5b79578ce56bd157f29a9056f0d6047465737416a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d6173736574010000000000000000000000000000ec0311";
        assert_eq!(c_pc_hex, c_pc_expected)
    }
}
