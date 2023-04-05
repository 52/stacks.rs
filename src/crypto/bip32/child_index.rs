use crate::crypto::bip32::Error;

pub(crate) const HARDENED_OFFSET: u32 = 0x80000000;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum ChildIndex {
    Normal { index: u32 },
    Hardened { index: u32 },
}

impl ChildIndex {
    pub(crate) fn from_normal(i: u32) -> Result<Self, Error> {
        if i >= HARDENED_OFFSET {
            return Err(Error::InvalidChildIndex(i));
        }

        Ok(ChildIndex::Normal { index: i })
    }

    pub(crate) fn from_hardened(i: u32) -> Result<Self, Error> {
        if i >= HARDENED_OFFSET {
            return Err(Error::InvalidChildIndex(i));
        }

        Ok(ChildIndex::Hardened {
            index: i + HARDENED_OFFSET,
        })
    }

    pub(crate) fn raw(self) -> u32 {
        match self {
            ChildIndex::Normal { index } => index,
            ChildIndex::Hardened { index } => index,
        }
    }

    pub(crate) fn is_hardened(self) -> bool {
        match self {
            ChildIndex::Normal { .. } => false,
            ChildIndex::Hardened { .. } => true,
        }
    }
}

impl std::fmt::Display for ChildIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_hardened() {
            write!(f, "{}'", self.raw() - HARDENED_OFFSET)
        } else {
            write!(f, "{}", self.raw())
        }
    }
}

impl std::str::FromStr for ChildIndex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hardened = s.ends_with('\'');
        let index = s
            .trim_end_matches('\'')
            .parse::<u32>()
            .map_err(|_| Error::InvalidChildIndexString)?;

        if hardened {
            ChildIndex::from_hardened(index)
        } else {
            ChildIndex::from_normal(index)
        }
    }
}

impl From<u32> for ChildIndex {
    fn from(i: u32) -> Self {
        if i & HARDENED_OFFSET != 0 {
            ChildIndex::Hardened {
                index: i ^ HARDENED_OFFSET,
            }
        } else {
            ChildIndex::Normal { index: i }
        }
    }
}

impl From<ChildIndex> for u32 {
    fn from(i: ChildIndex) -> u32 {
        i.raw()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_child_index_parse() {
        let arr = vec![
            "42'".parse::<ChildIndex>().unwrap(),
            "42".parse::<ChildIndex>().unwrap(),
        ];

        assert_eq!(
            arr[0],
            ChildIndex::Hardened {
                index: 42 | HARDENED_OFFSET
            }
        );
        assert_eq!(arr[1], ChildIndex::Normal { index: 42 });

        assert_eq!(arr[0].raw(), 42 | HARDENED_OFFSET);
        assert_eq!(arr[1].raw(), 42);

        assert_eq!(arr[0].to_string(), "42'");
        assert_eq!(arr[1].to_string(), "42");

        assert_eq!(arr[0].is_hardened(), true);
        assert_eq!(arr[1].is_hardened(), false);
    }

    #[test]
    fn test_child_index_error() {
        let invalid_char = "42!".parse::<ChildIndex>();
        assert_eq!(invalid_char, Err(Error::InvalidChildIndexString));

        let invalid_index = HARDENED_OFFSET;

        assert_eq!(
            ChildIndex::from_normal(invalid_index),
            Err(Error::InvalidChildIndex(invalid_index))
        );

        assert_eq!(
            ChildIndex::from_hardened(invalid_index),
            Err(Error::InvalidChildIndex(invalid_index))
        );
    }
}
