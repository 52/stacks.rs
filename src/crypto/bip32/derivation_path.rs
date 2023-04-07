use crate::crypto::bip32::child_index::ChildIndex;
use crate::crypto::bip32::Error;

pub(crate) const DERIVATION_PATH_PREFIX: &str = "m";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DerivationPath {
    path: Vec<ChildIndex>,
}

impl IntoIterator for DerivationPath {
    type Item = ChildIndex;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.path.into_iter()
    }
}

impl std::fmt::Display for DerivationPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{DERIVATION_PATH_PREFIX}")?;

        for child in &self.path {
            write!(f, "/{child}")?;
        }

        Ok(())
    }
}

impl std::str::FromStr for DerivationPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('/');

        if iter.next() != Some(DERIVATION_PATH_PREFIX) {
            return Err(Error::InvalidDerivationPath);
        }

        let path = iter
            .map(str::parse)
            .collect::<Result<Vec<ChildIndex>, Self::Err>>()?;

        Ok(DerivationPath { path })
    }
}

pub(crate) trait IntoDerivationPath {
    fn into_derivation_path(self) -> Result<DerivationPath, Error>;
}

impl IntoDerivationPath for String {
    fn into_derivation_path(self) -> Result<DerivationPath, Error> {
        self.parse()
    }
}

impl IntoDerivationPath for &str {
    fn into_derivation_path(self) -> Result<DerivationPath, Error> {
        self.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::bip32::child_index::HARDENED_OFFSET;

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
            let parsed: DerivationPath = path.into_derivation_path().unwrap();
            assert_eq!(parsed.to_string(), path);
        }
    }

    #[test]
    fn test_derivation_path_indices() {
        let path: DerivationPath = "m/1'/2'/3'/0".into_derivation_path().unwrap();

        assert_eq!(
            path,
            DerivationPath {
                path: vec![
                    ChildIndex::Hardened {
                        index: 1 | HARDENED_OFFSET
                    },
                    ChildIndex::Hardened {
                        index: 2 | HARDENED_OFFSET
                    },
                    ChildIndex::Hardened {
                        index: 3 | HARDENED_OFFSET
                    },
                    ChildIndex::Normal { index: 0 },
                ],
            }
        );
    }
}
