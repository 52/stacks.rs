// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use std::fmt::Debug;
use std::fmt::Display;

use dyn_clone::clone_trait_object;
use dyn_clone::DynClone;

use crate::clarity;
use crate::clarity::macros::impl_clarity_primitive;
use crate::clarity::Cast;
use crate::clarity::Clarity;
use crate::clarity::Codec;
use crate::clarity::LengthPrefixedStr;
use crate::clarity::PrincipalContract;
use crate::clarity::PrincipalStandard;
use crate::clarity::CLARITY_TYPE_NON_STD;

/// The standard STX condition type.
pub(crate) const POST_CONDITION_TYPE_STX: u8 = 0x00;
/// The fungible condition type.
pub(crate) const POST_CONDITION_TYPE_FUNGIBLE: u8 = 0x01;
/// The non-fungible condition type.
pub(crate) const POST_CONDITION_TYPE_NON_FUNGIBLE: u8 = 0x02;
/// The standard principal type.
pub(crate) const POST_CONDITION_PRINCIPAL_STD: u8 = 0x02;
/// The contract principal type.
pub(crate) const POST_CONDITION_PRINCIPAL_CON: u8 = 0x03;

/// Convenience macro for creating post-conditions.
#[macro_export]
macro_rules! post_condition {
    (@box $e:expr) => (Box::new($e) as Box<dyn $crate::transaction::Condition>);
    () => ($crate::transaction::PostConditions::new(Vec::new()));
    ($(($type:ident, $($args:tt)*)),* $(,)?) => {{
        let mut tmp = Vec::new();
        $(tmp.push($crate::post_condition!(@gen $type, $($args)*));)*
        $crate::transaction::PostConditions::new(tmp)
    }};
    ($type:ident, $($args:tt)*) => ($crate::post_condition!(@gen $type, $($args)*));
    (@gen STXCondition, $($args:tt)*) => ($crate::post_condition!(@box $crate::transaction::STXPostCondition::new($($args)*)));
    (@gen FungibleCondition, $($args:tt)*) => ($crate::post_condition!(@box $crate::transaction::FungiblePostCondition::new($($args)*)));
    (@gen NonFungibleCondition, $($args:tt)*) => ($crate::post_condition!(@box $crate::transaction::NonFungiblePostCondition::new($($args)*)));
}

/// Marker trait for post-conditions.
pub trait Condition: Codec + DynClone + Send + Sync + Debug {}
clone_trait_object!(Condition);

/// The post-condition code.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConditionCode {
    /// Equal condition.
    EQ = 0x01,
    /// Greater than condition.
    GT = 0x02,
    /// Greater than or equal condition.
    GTE = 0x03,
    /// Less than condition.
    LT = 0x04,
    /// Less than or equal condition.
    LTE = 0x05,
    /// Has not or doesn't own condition. (Non-Fungible)
    HasNot = 0x10,
    /// Has or owns condition. (Non-Fungible)
    Has = 0x11,
}

/// The post-condition mode.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PostConditionMode {
    /// Allow mode, this turns off all post-conditions.
    Allow = 0x01,
    /// Deny mode, this turns on all post-conditions.
    Deny = 0x02,
}

impl_clarity_primitive!(
    PostConditions,
    Vec<Box<dyn Condition>>,
    CLARITY_TYPE_NON_STD
);

impl Codec for PostConditions {
    fn encode(&self) -> Result<Vec<u8>, clarity::Error> {
        let mut buff = vec![];
        buff.extend_from_slice(&u32::try_from(self.__value.len())?.to_be_bytes());

        for value in &self.__value {
            buff.extend_from_slice(&value.encode()?);
        }

        Ok(buff)
    }

    #[allow(unused_variables)]
    fn decode(bytes: &[u8]) -> Result<Self, clarity::Error>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl Default for PostConditions {
    fn default() -> Self {
        PostConditions::new(Vec::default())
    }
}

impl Display for PostConditions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.__value)
    }
}

impl Debug for PostConditions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.__value)
    }
}

impl Clone for PostConditions {
    fn clone(&self) -> Self {
        Self::new(self.__value.clone())
    }
}

impl IntoIterator for PostConditions {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = Box<dyn Condition>;

    fn into_iter(self) -> Self::IntoIter {
        self.__value.into_iter()
    }
}

/// The post-condition for native STX tokens.
#[derive(Debug, Clone)]
pub struct STXPostCondition {
    /// The principal address of the condition.
    address: Box<dyn Clarity>,
    /// The amount of the condition.
    amount: u64,
    /// The condition code of the condition.
    code: ConditionCode,
}

impl STXPostCondition {
    /// Creates a new `STXPostCondition`.
    pub fn new<T>(address: T, amount: u64, code: ConditionCode) -> Self
    where
        T: Clarity,
    {
        Self {
            address: Box::new(address),
            amount,
            code,
        }
    }
}

impl Codec for STXPostCondition {
    fn encode(&self) -> Result<Vec<u8>, clarity::Error> {
        let mut buff = vec![POST_CONDITION_TYPE_STX];

        if let Ok(addr) = self.address.cast_as::<PrincipalStandard>() {
            buff.push(POST_CONDITION_PRINCIPAL_STD);
            buff.extend_from_slice(&addr.encode()?[1..]);
        } else if let Ok(addr) = self.address.cast_as::<PrincipalContract>() {
            buff.push(POST_CONDITION_PRINCIPAL_CON);
            buff.extend_from_slice(&addr.encode()?[1..]);
        } else {
            return Err(clarity::Error::BadDowncast);
        }

        buff.push(self.code as u8);
        buff.extend_from_slice(&self.amount.to_be_bytes());
        Ok(buff)
    }

    #[allow(unused_variables)]
    fn decode(bytes: &[u8]) -> Result<Self, clarity::Error>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl Condition for STXPostCondition {}

/// The post-condition for fungible tokens.
#[derive(Debug, Clone)]
pub struct FungiblePostCondition {
    /// The principal address of the condition.
    address: Box<dyn Clarity>,
    /// The amount of the condition.
    amount: u64,
    /// The condition code of the condition.
    code: ConditionCode,
    /// The asset info of the condition.
    info: AssetInfo,
}

impl FungiblePostCondition {
    /// Creates a new `FungiblePostCondition`.
    pub fn new<T>(address: T, amount: u64, code: ConditionCode, info: AssetInfo) -> Self
    where
        T: Clarity,
    {
        Self {
            address: Box::new(address),
            amount,
            code,
            info,
        }
    }
}

impl Codec for FungiblePostCondition {
    fn encode(&self) -> Result<Vec<u8>, clarity::Error> {
        let mut buff = vec![POST_CONDITION_TYPE_FUNGIBLE];

        if let Ok(addr) = self.address.cast_as::<PrincipalStandard>() {
            buff.push(POST_CONDITION_PRINCIPAL_STD);
            buff.extend_from_slice(&addr.encode()?[1..]);
        } else if let Ok(addr) = self.address.cast_as::<PrincipalContract>() {
            buff.push(POST_CONDITION_PRINCIPAL_CON);
            buff.extend_from_slice(&addr.encode()?[1..]);
        } else {
            return Err(clarity::Error::BadDowncast);
        }

        buff.extend_from_slice(&self.info.encode()?);
        buff.push(self.code as u8);
        buff.extend_from_slice(&self.amount.to_be_bytes());
        Ok(buff)
    }

    #[allow(unused_variables)]
    fn decode(bytes: &[u8]) -> Result<Self, clarity::Error>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl Condition for FungiblePostCondition {}

/// The post-condition for non-fungible tokens.
#[derive(Debug, Clone)]
pub struct NonFungiblePostCondition {
    /// The principal address of the condition.
    pub address: Box<dyn Clarity>,
    /// A clarity value that names the token instance.
    pub name: Box<dyn Clarity>,
    /// The condition code of the condition.
    pub code: ConditionCode,
    /// The asset info of the condition.
    pub info: AssetInfo,
}

impl NonFungiblePostCondition {
    /// Creates a new `NonFungiblePostCondition`.
    pub fn new<T, S>(address: T, name: S, code: ConditionCode, info: AssetInfo) -> Self
    where
        T: Clarity,
        S: Clarity,
    {
        Self {
            address: Box::new(address),
            name: Box::new(name),
            code,
            info,
        }
    }
}

impl Codec for NonFungiblePostCondition {
    fn encode(&self) -> Result<Vec<u8>, clarity::Error> {
        let mut buff = vec![POST_CONDITION_TYPE_NON_FUNGIBLE];

        if let Ok(addr) = self.address.cast_as::<PrincipalStandard>() {
            buff.push(POST_CONDITION_PRINCIPAL_STD);
            buff.extend_from_slice(&addr.encode()?[1..]);
        } else if let Ok(addr) = self.address.cast_as::<PrincipalContract>() {
            buff.push(POST_CONDITION_PRINCIPAL_CON);
            buff.extend_from_slice(&addr.encode()?[1..]);
        } else {
            return Err(clarity::Error::BadDowncast);
        }

        buff.extend_from_slice(&self.info.encode()?);
        buff.extend_from_slice(&self.name.encode()?);
        buff.push(self.code as u8);
        Ok(buff)
    }

    #[allow(unused_variables)]
    fn decode(bytes: &[u8]) -> Result<Self, clarity::Error>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl Condition for NonFungiblePostCondition {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetInfo {
    /// The principal address of the asset.
    address: PrincipalStandard,
    /// The contract name of the asset.
    name: LengthPrefixedStr,
    /// The name of the asset.
    asset: LengthPrefixedStr,
}

impl AssetInfo {
    pub fn new<T>(address: T, name: T, asset: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            address: PrincipalStandard::new(address.into()),
            name: LengthPrefixedStr::new(name.into()),
            asset: LengthPrefixedStr::new(asset.into()),
        }
    }
}

impl Codec for AssetInfo {
    fn encode(&self) -> Result<Vec<u8>, clarity::Error> {
        let mut buff = vec![];
        buff.extend_from_slice(&self.address.encode()?[1..]);
        buff.extend_from_slice(&self.name.encode()?);
        buff.extend_from_slice(&self.asset.encode()?);
        Ok(buff)
    }

    #[allow(unused_variables)]
    fn decode(bytes: &[u8]) -> Result<Self, clarity::Error>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hex::bytes_to_hex;

    #[test]
    fn test_transaction_conditions_encode() {
        let (addr, name, info) = get_test_data();

        let prefixed = clarity!(LengthPrefixedStr, name);

        let conditions = post_condition!(
            (
                STXCondition,
                clarity!(PrincipalStandard, addr),
                1000000,
                ConditionCode::GTE
            ),
            (
                STXCondition,
                clarity!(PrincipalContract, addr, name),
                1000000,
                ConditionCode::GTE
            ),
            (
                NonFungibleCondition,
                clarity!(PrincipalStandard, addr),
                prefixed,
                ConditionCode::Has,
                info.clone()
            ),
            (
                FungibleCondition,
                clarity!(PrincipalContract, addr, name),
                1000000,
                ConditionCode::GTE,
                info.clone()
            )
        );

        let encoded = conditions.encode().unwrap();
        let hex = bytes_to_hex(&encoded);

        let expected = "00000004000216a5d9d331000f5b79578ce56bd157f29a9056f0d60300000000000f4240000316a5d9d331000f5b79578ce56bd157f29a9056f0d604746573740300000000000f4240020216a5d9d331000f5b79578ce56bd157f29a9056f0d616a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d6173736574047465737411010316a5d9d331000f5b79578ce56bd157f29a9056f0d6047465737416a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d61737365740300000000000f4240";
        assert_eq!(hex, expected);
    }

    #[test]
    fn test_transaction_conditions_stx_encode() {
        let (addr, name, _) = get_test_data();

        let std = clarity!(PrincipalStandard, addr);
        let con = clarity!(PrincipalContract, addr, name);

        let std_pc = STXPostCondition::new(std, 1000000, ConditionCode::GTE);

        let std_pc_encoded = std_pc.encode().unwrap();
        let std_pc_hex = bytes_to_hex(&std_pc_encoded);

        let std_pc_expected = "000216a5d9d331000f5b79578ce56bd157f29a9056f0d60300000000000f4240";
        assert_eq!(std_pc_hex, std_pc_expected);

        let con_pc = STXPostCondition::new(con, 1000000, ConditionCode::GTE);

        let con_pc_encoded = con_pc.encode().unwrap();
        let con_pc_hex = bytes_to_hex(&con_pc_encoded);

        let con_pc_expected =
            "000316a5d9d331000f5b79578ce56bd157f29a9056f0d604746573740300000000000f4240";
        assert_eq!(con_pc_hex, con_pc_expected)
    }

    #[test]
    fn test_transaction_conditions_ft_encode() {
        let (addr, name, info) = get_test_data();

        let std = clarity!(PrincipalStandard, addr);
        let con = clarity!(PrincipalContract, addr, name);

        let std_pc = FungiblePostCondition::new(std, 1000000, ConditionCode::EQ, info.clone());

        let std_pc_encoded = std_pc.encode().unwrap();
        let std_pc_hex = bytes_to_hex(&std_pc_encoded);

        let std_pc_expected = "010216a5d9d331000f5b79578ce56bd157f29a9056f0d616a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d61737365740100000000000f4240";
        assert_eq!(std_pc_hex, std_pc_expected);

        let con_pc = FungiblePostCondition::new(con, 1000000, ConditionCode::EQ, info);

        let con_pc_encoded = con_pc.encode().unwrap();
        let con_pc_hex = bytes_to_hex(&con_pc_encoded);

        let con_pc_expected = "010316a5d9d331000f5b79578ce56bd157f29a9056f0d6047465737416a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d61737365740100000000000f4240";
        assert_eq!(con_pc_hex, con_pc_expected)
    }

    #[test]
    fn test_transaction_conditions_nft_encode() {
        let (addr, name, info) = get_test_data();

        let std = clarity!(PrincipalStandard, addr);
        let con = clarity!(PrincipalContract, addr, name);
        let prefixed = clarity!(LengthPrefixedStr, name);

        let std_pc =
            NonFungiblePostCondition::new(std, prefixed.clone(), ConditionCode::Has, info.clone());

        let std_pc_encoded = std_pc.encode().unwrap();
        let std_pc_hex = bytes_to_hex(&std_pc_encoded);

        let std_pc_expected = "020216a5d9d331000f5b79578ce56bd157f29a9056f0d616a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d6173736574047465737411";
        assert_eq!(std_pc_hex, std_pc_expected);

        let con_pc = NonFungiblePostCondition::new(con, prefixed, ConditionCode::Has, info);

        let con_pc_encoded = con_pc.encode().unwrap();
        let con_pc_hex = bytes_to_hex(&con_pc_encoded);

        let con_pc_expected = "020316a5d9d331000f5b79578ce56bd157f29a9056f0d6047465737416a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d6173736574047465737411";
        assert_eq!(con_pc_hex, con_pc_expected)
    }

    #[test]
    fn test_transaction_conditions_info_encode() {
        let (_, _, info) = get_test_data();
        let encoded = info.encode().unwrap();
        let hex = bytes_to_hex(&encoded);
        let expected =
            "16a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d6173736574";
        assert_eq!(hex, expected);
    }

    fn get_test_data() -> (String, String, AssetInfo) {
        let addr = "SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B";
        let name = "test";

        let info = AssetInfo::new(
            "SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B",
            "my-contract",
            "my-asset",
        );

        (addr.to_string(), name.to_string(), info)
    }
}
