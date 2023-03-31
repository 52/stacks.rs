use std::fmt::Display;
use std::str::FromStr;

use crate::crypto_extras::bip32::key_index::KeyIndex;
use crate::prelude::*;

const DERIVATION_PATH_PREFIX: &'static str = "m";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DerivationPath {
    path: Vec<KeyIndex>,
}

impl IntoIterator for DerivationPath {
    type Item = KeyIndex;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.path.into_iter()
    }
}

impl Display for DerivationPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", DERIVATION_PATH_PREFIX)?;

        for child in &self.path {
            write!(f, "/{}", child)?;
        }

        Ok(())
    }
}

impl FromStr for DerivationPath {
    type Err = Error;

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        let mut path = path.split("/");

        if path.next() != Some(DERIVATION_PATH_PREFIX) {
            return Err(Error::Generic);
        }

        Ok(DerivationPath {
            path: path.map(str::parse).collect::<Result<_>>()?,
        })
    }
}

impl TryFrom<String> for DerivationPath {
    type Error = Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.parse() {
            Ok(path) => Ok(path),
            Err(_) => Err(Error::Generic),
        }
    }
}

impl TryFrom<&str> for DerivationPath {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.parse() {
            Ok(path) => Ok(path),
            Err(_) => Err(Error::Generic),
        }
    }
}

pub(crate) trait IntoDerivationPath {
    fn into_path(self) -> Result<DerivationPath>;
}

impl IntoDerivationPath for DerivationPath {
    fn into_path(self) -> Result<DerivationPath> {
        Ok(self)
    }
}

impl IntoDerivationPath for String {
    fn into_path(self) -> Result<DerivationPath> {
        self.try_into()
    }
}

impl IntoDerivationPath for &str {
    fn into_path(self) -> Result<DerivationPath> {
        self.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::DerivationPath;
    use super::KeyIndex;
    use crate::crypto_extras::bip32::key_index::HARDENED_OFFSET;

    #[test]
    fn test_derivation_path() {
        let paths = vec![
            "m",
            "m/0",
            "m/0/2147483647'",
            "m/0/2147483647'/1",
            "m/0/2147483647'/1/2147483646'",
            "m/0/2147483647'/1/2147483646'/2",
        ];

        for path in paths {
            let parsed: DerivationPath = path.try_into().unwrap();
            assert_eq!(parsed.to_string(), path);
        }
    }

    #[test]
    fn test_derivation_path_indecies() {
        let path: DerivationPath = "m/1'/2'/3'/0".try_into().unwrap();

        assert_eq!(
            path,
            DerivationPath {
                path: vec![
                    KeyIndex(1 | HARDENED_OFFSET),
                    KeyIndex(2 | HARDENED_OFFSET),
                    KeyIndex(3 | HARDENED_OFFSET),
                    KeyIndex(0),
                ],
            }
        );
    }
}
