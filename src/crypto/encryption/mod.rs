pub(crate) mod hash160;
pub(crate) mod sha;

pub(crate) trait FromSlice
where
    Self: Sized + AsRef<[u8]>,
{
    fn from_slice(value: &[u8]) -> Self;
}
