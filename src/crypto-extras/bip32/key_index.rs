use std::fmt::Display;
use std::str::FromStr;

use crate::prelude::*;

pub(crate) const INDEX_BYTE_SIZE: usize = 4;
pub(crate) const HARDENED_OFFSET: u32 = 0x80000000;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct KeyIndex(pub(crate) u32);

impl KeyIndex {
    pub(crate) fn new(i: u32, hardened: bool) -> Result<Self> {
        if i >= HARDENED_OFFSET {
            return Err(Error::Generic);
        }

        if hardened {
            Ok(KeyIndex(i | HARDENED_OFFSET))
        } else {
            Ok(KeyIndex(i))
        }
    }

    pub(crate) fn raw(self) -> u32 {
        self.0
    }

    pub(crate) fn bytes(self) -> [u8; INDEX_BYTE_SIZE] {
        self.raw().to_be_bytes()
    }

    pub(crate) fn is_hardened(self) -> bool {
        self.0 & HARDENED_OFFSET != 0
    }
}

impl Display for KeyIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_hardened() {
            write!(f, "{}'", self.raw() - HARDENED_OFFSET)
        } else {
            write!(f, "{}", self.raw())
        }
    }
}

impl FromStr for KeyIndex {
    type Err = Error;

    fn from_str(child: &str) -> Result<Self> {
        let hardened = child.ends_with('\'');

        let index = child
            .trim_end_matches('\'')
            .parse::<u32>()
            .map_err(|_| Error::Generic)?;

        KeyIndex::new(index, hardened)
    }
}

impl From<u32> for KeyIndex {
    fn from(i: u32) -> Self {
        KeyIndex(i)
    }
}

impl From<KeyIndex> for u32 {
    fn from(i: KeyIndex) -> u32 {
        i.raw()
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use super::KeyIndex;
    use super::HARDENED_OFFSET;

    #[test]
    fn parse_hardened() {
        let index = "42'".parse::<KeyIndex>().unwrap();

        assert_eq!(index, KeyIndex(42 | HARDENED_OFFSET));
        assert_eq!(index.raw(), 42 | HARDENED_OFFSET);
    }

    #[test]
    fn parse_not_hardened() {
        let index = "42".parse::<KeyIndex>().unwrap();

        assert_eq!(index, KeyIndex(42));
        assert_eq!(index.raw(), 42);
    }

    #[test]
    fn string_hardened() {
        let index = "42'".parse::<KeyIndex>().unwrap();
        assert_eq!(index.to_string(), "42'");
    }

    #[test]
    fn string_not_hardened() {
        let index = "42".parse::<KeyIndex>().unwrap();
        assert_eq!(index.to_string(), "42");
    }

    #[test]
    fn throws_invalid() {
        let index = "42!".parse::<KeyIndex>();
        assert_eq!(index, Err(Error::Generic));
    }

    #[test]
    fn throws_overflow() {
        let index = HARDENED_OFFSET;
        assert_eq!(KeyIndex::new(index, false), Err(Error::Generic));
        assert_eq!(KeyIndex::new(index, true), Err(Error::Generic));
    }
}
