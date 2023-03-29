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

pub(crate) trait IntoDerivationPath {
    fn into(self) -> Result<DerivationPath, Error>;
}

impl IntoDerivationPath for DerivationPath {
    fn into(self) -> Result<DerivationPath, Error> {
        Ok(self)
    }
}

impl IntoDerivationPath for String {
    fn into(self) -> Result<DerivationPath, Error> {
        self.parse()
    }
}

impl IntoDerivationPath for &str {
    fn into(self) -> Result<DerivationPath, Error> {
        self.parse()
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
            assert_eq!(path.parse::<DerivationPath>().unwrap().to_string(), path);
        }
    }

    #[test]
    fn test_derivation_path_indecies() {
        let path = "m/1'/2'/3'/0".parse::<DerivationPath>().unwrap();

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
