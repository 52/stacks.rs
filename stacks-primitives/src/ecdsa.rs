// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use core::fmt;
use core::marker;
use core::str;

use crate::__private::Box;
use crate::__private::String;
use crate::__private::Vec;
use crate::bytes::Bytes;
use crate::bytes::FixedBytes;
use crate::bytes::B256;
use crate::bytes::B8;
use crate::hash::H256;

/// Re-exports of [`k256`] types.
///
/// For more details, see the [`k256`] crate documentation.
pub mod k256 {
    pub use k256::ecdsa::Error;
    pub use k256::ecdsa::RecoveryId;
    pub use k256::ecdsa::Signature;
    pub use k256::ecdsa::SigningKey;
    pub use k256::ecdsa::VerifyingKey;
    pub use k256::elliptic_curve::bigint::Encoding;
    pub use k256::elliptic_curve::Curve;
    pub use k256::Secp256k1;
    pub use k256::U256;
}

/// Type alias for [`k256::Error`].
///
/// For more details, see the [`k256::Error`] documentation.
pub type Error = k256::Error;

/// Type wrapper around [`k256::SigningKey`].
///
/// # Layout
///
/// The memory layout is identical to that of [`k256::SigningKey`]:
///
/// ```rust
/// # use core::mem::align_of;
/// # use core::mem::size_of;
/// # use stacks_primitives::ecdsa::k256;
/// # use stacks_primitives::ecdsa::PrivateKey;
/// assert_eq!(size_of::<PrivateKey>(), size_of::<k256::SigningKey>());
/// assert_eq!(align_of::<PrivateKey>(), align_of::<k256::SigningKey>());
/// ```
///
/// # Hex
///
/// Hex-encoded strings can be parsed into [`PrivateKey`] via [`hex::FromHex`]:
///
/// ```rust,no_run
/// # use stacks_primitives::ecdsa::PrivateKey;
/// # use stacks_primitives::hex::FromHex;
/// let hex = "0x8356aef9e7d87e971d2812f6c0b273ee102c12dad1...";
/// let key = PrivateKey::from_hex(hex);
/// ```
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct PrivateKey {
    /// The underlying [`k256::SigningKey`].
    ///
    /// For more details, see the [`k256::SigningKey`] documentation.
    key: k256::SigningKey,
}

impl PrivateKey {
    /// Creates a new [`PrivateKey`] from a [`k256::SigningKey`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rand_core::OsRng;
    /// use stacks_primitives::ecdsa::k256;
    /// use stacks_primitives::ecdsa::PrivateKey;
    ///
    /// let k256 = k256::SigningKey::random(&mut OsRng);
    /// let key = PrivateKey::new(k256);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(key: k256::SigningKey) -> Self {
        Self { key }
    }

    /// Returns the underlying [`k256::SigningKey`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stacks_primitives::ecdsa::PrivateKey;
    ///
    /// let key = PrivateKey::random();
    /// let raw = key.raw();
    /// ```
    #[inline]
    #[must_use]
    pub fn raw(self) -> k256::SigningKey {
        self.key
    }

    /// Derives the corresponding [`PublicKey`] from this [`PrivateKey`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stacks_primitives::ecdsa::PrivateKey;
    /// use stacks_primitives::ecdsa::PublicKey;
    ///
    /// let key = PrivateKey::random();
    /// let public: PublicKey = key.public();
    /// ```
    #[inline]
    #[must_use]
    pub fn public(&self) -> PublicKey {
        let key = k256::VerifyingKey::from(self.key.clone());
        PublicKey { key }
    }

    /// Creates a [`PrivateKey`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the bytes do not represent a valid key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::ecdsa::Error;
    /// use stacks_primitives::ecdsa::PrivateKey;
    ///
    /// let bytes = PrivateKey::random().to_bytes();
    /// let key = PrivateKey::from_slice(&bytes[..])?;
    /// assert_eq!(key.to_bytes(), bytes);
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        let key = k256::SigningKey::from_slice(bytes)?;
        Ok(Self { key })
    }

    /// Creates a [`PrivateKey`] from a fixed-length byte array.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the bytes do not represent a valid key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::ecdsa::Error;
    /// use stacks_primitives::ecdsa::PrivateKey;
    ///
    /// let bytes = PrivateKey::random().to_bytes();
    /// let key = PrivateKey::from_bytes(bytes)?;
    /// assert_eq!(key.to_bytes(), bytes);
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn from_bytes(bytes: B256) -> Result<Self, Error> {
        let key = k256::SigningKey::from_bytes(&bytes.raw().into())?;
        Ok(Self { key })
    }

    /// Converts the [`PrivateKey`] to a fixed-length byte array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::ecdsa::Error;
    /// use stacks_primitives::ecdsa::PrivateKey;
    ///
    /// let key = PrivateKey::random();
    /// let bytes = key.to_bytes();
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    #[must_use]
    pub fn to_bytes(&self) -> B256 {
        B256::from(<[u8; 32]>::from(self.key.to_bytes()))
    }

    /// Generates a new random [`PrivateKey`] using [`rand_core::OsRng`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stacks_primitives::ecdsa::PrivateKey;
    ///
    /// let key = PrivateKey::random();
    /// ```
    #[inline]
    #[must_use]
    #[cfg(feature = "rand")]
    pub fn random() -> Self {
        Self::random_with(&mut rand_core::OsRng)
    }

    /// Generates a new random [`PrivateKey`] using the provided RNG.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stacks_primitives::ecdsa::PrivateKey;
    ///
    /// let mut rng = rand_core::OsRng;
    /// let key = PrivateKey::random_with(&mut rng);
    /// ```
    #[inline]
    #[must_use]
    #[cfg(feature = "rand")]
    pub fn random_with<R>(rng: &mut R) -> Self
    where
        R: rand_core::CryptoRng + rand_core::RngCore,
    {
        let key = k256::SigningKey::random(rng);
        Self { key }
    }

    /// Signs a prehash with the private key, returning a [`Signature`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the signing operation fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::ecdsa::Error;
    /// use stacks_primitives::ecdsa::PrivateKey;
    /// use stacks_primitives::hash::Sha256;
    ///
    /// let key = PrivateKey::random();
    /// let message = Sha256::compute(b"usque ad finem");
    /// let signature = key.sign(message)?;
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn sign(&self, prehash: H256) -> Result<Signature, Error> {
        let (s, r) = self.key.sign_prehash_recoverable(&prehash[..])?;
        Ok(Signature::from_k256(s, r))
    }
}

impl str::FromStr for PrivateKey {
    type Err = Error;

    /// Creates a [`PrivateKey`] from a string.
    ///
    /// This method delegates to [`hex::FromHex`].
    #[inline]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        hex::FromHex::from_hex(str)
    }
}

impl hex::FromHex for PrivateKey {
    type Error = Error;

    /// Creates a [`PrivateKey`] from a hex-encoded byte sequence.
    ///
    /// Expects a 32-byte key encoded as a prefixed or unprefixed hex string.
    #[inline]
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        let decoded = hex::decode(hex).map_err(|_| Error::new())?;
        let mut bytes = B256::zero();
        bytes.copy_from_slice(&decoded);
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&str> for PrivateKey {
    type Error = Error;

    #[inline]
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        <PrivateKey as hex::FromHex>::from_hex(str)
    }
}

impl TryFrom<String> for PrivateKey {
    type Error = Error;

    #[inline]
    fn try_from(str: String) -> Result<Self, Self::Error> {
        <PrivateKey as hex::FromHex>::from_hex(str)
    }
}

impl TryFrom<&[u8]> for PrivateKey {
    type Error = Error;

    #[inline]
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::from_slice(bytes)
    }
}

impl TryFrom<Bytes> for PrivateKey {
    type Error = Error;

    #[inline]
    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        Self::from_slice(&bytes)
    }
}

impl TryFrom<[u8; 32]> for PrivateKey {
    type Error = Error;

    #[inline]
    fn try_from(bytes: [u8; 32]) -> Result<Self, Self::Error> {
        Self::from_bytes(bytes.into())
    }
}

impl TryFrom<FixedBytes<32>> for PrivateKey {
    type Error = Error;

    #[inline]
    fn try_from(bytes: FixedBytes<32>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.raw())
    }
}

impl TryFrom<Box<[u8]>> for PrivateKey {
    type Error = Error;

    #[inline]
    fn try_from(bytes: Box<[u8]>) -> Result<Self, Self::Error> {
        Self::from_slice(bytes.as_ref())
    }
}

impl TryFrom<Vec<u8>> for PrivateKey {
    type Error = Error;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_slice(bytes.as_slice())
    }
}

/// Type wrapper around [`k256::VerifyingKey`].
///
/// # Layout
///
/// The memory layout is identical to that of [`k256::VerifyingKey`]:
///
/// ```rust
/// # use core::mem::align_of;
/// # use core::mem::size_of;
/// # use stacks_primitives::ecdsa::k256;
/// # use stacks_primitives::ecdsa::PublicKey;
/// assert_eq!(size_of::<PublicKey>(), size_of::<k256::VerifyingKey>());
/// assert_eq!(align_of::<PublicKey>(), align_of::<k256::VerifyingKey>());
/// ```
///
/// # Format
///
/// [`PublicKey`] supports [`fmt::LowerHex`] and [`fmt::UpperHex`], producing a
/// `0x` prefixed string of its compressed encoded bytes:
///
/// ```rust,no_run
/// # use stacks_primitives::ecdsa::PublicKey;
/// # use stacks_primitives::hex::FromHex;
/// let hex = "0x0303e6609bd278c27024fb0f2a4e9a331bc4b24a0e4c44f...";
/// let key = PublicKey::from_hex(hex);
/// ```
///
/// ## Hex
///
/// Hex-encoded strings can be parsed into [`PublicKey`] via [`hex::FromHex`]:
///
/// ```rust,no_run
/// # use stacks_primitives::ecdsa::PublicKey;
/// # use stacks_primitives::hex::FromHex;
/// let hex = "0x0303e6609bd278c27024fb0f2a4e9a331bc4b24a0e4c44f...";
/// let key = PublicKey::from_hex(hex);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PublicKey {
    /// The underlying [`k256::VerifyingKey`].
    ///
    /// For more details, see the [`k256::VerifyingKey`] documentation.
    key: k256::VerifyingKey,
}

impl PublicKey {
    /// Creates a new [`PublicKey`] from a [`k256::VerifyingKey`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rand_core::OsRng;
    /// use stacks_primitives::ecdsa::k256;
    /// use stacks_primitives::ecdsa::PublicKey;
    ///
    /// let sk = k256::SigningKey::random(&mut OsRng);
    /// let k256 = k256::VerifyingKey::from(sk);
    /// let key = PublicKey::new(k256);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(key: k256::VerifyingKey) -> Self {
        Self { key }
    }

    /// Returns the underlying [`k256::VerifyingKey`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stacks_primitives::ecdsa::PrivateKey;
    /// use stacks_primitives::ecdsa::PublicKey;
    ///
    /// let pk = PrivateKey::random();
    /// let key: PublicKey = pk.public();
    /// let raw = key.raw();
    /// ```
    #[inline]
    #[must_use]
    pub const fn raw(self) -> k256::VerifyingKey {
        self.key
    }

    /// Creates a [`PublicKey`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the bytes do not represent a valid key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::ecdsa::Error;
    /// use stacks_primitives::ecdsa::PrivateKey;
    /// use stacks_primitives::ecdsa::PublicKey;
    ///
    /// let pk = PrivateKey::random();
    /// let bytes = pk.public().to_bytes(true);
    /// let key = PublicKey::from_bytes(&bytes)?;
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let key = k256::VerifyingKey::from_sec1_bytes(bytes)?;
        Ok(Self { key })
    }

    /// Converts the [`PublicKey`] to a byte sequence.
    ///
    /// # Notes
    ///
    /// - `compress == true`, yields the compressed form (33 bytes).
    /// - `compress == false`, yields the uncompressed form (65 bytes).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stacks_primitives::ecdsa::PrivateKey;
    /// use stacks_primitives::ecdsa::PublicKey;
    ///
    /// let pk = PrivateKey::random();
    /// let key: PublicKey = pk.public();
    ///
    /// let comp = key.to_bytes(true);
    /// assert_eq!(comp.len(), 33);
    ///
    /// let ucomp = key.to_bytes(false);
    /// assert_eq!(ucomp.len(), 65);
    /// ```
    #[inline]
    #[must_use]
    pub fn to_bytes(&self, compress: bool) -> Bytes {
        let point = self.key.to_encoded_point(compress);
        Bytes::from_slice(point.as_bytes())
    }

    /// Recovers a [`PublicKey`] from a prehash and [`Signature`].
    ///
    /// # Errors
    ///
    ///  Returns an [`Error`] if:
    ///
    /// - The signature is invalid.
    /// - The prehash is not a valid 32-byte hash.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::ecdsa::Error;
    /// use stacks_primitives::ecdsa::PrivateKey;
    /// use stacks_primitives::ecdsa::PublicKey;
    /// use stacks_primitives::hash::Sha256;
    ///
    /// let pk = PrivateKey::random();
    /// let message = Sha256::compute(b"usque ad finem");
    ///
    /// let signature = pk.sign(message)?;
    /// let key = PublicKey::recover(message, signature)?;
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn recover(prehash: H256, sig: Signature) -> Result<Self, Error> {
        let ((s, r), h) = (sig.to_k256()?, &prehash[..]);
        k256::VerifyingKey::recover_from_prehash(h, &s, r).map(Self::new)
    }

    /// Verifies a [`Signature`] against a prehash.
    ///
    /// # Notes
    ///
    /// The verification process involves the following steps:
    ///
    /// - Normalize the [`Signature`] to its low-S form to prevent malleability.
    /// - Recover the [`PublicKey`] from the normalized signature and message.
    /// - Assert that the recovered key matches the provided key.
    /// - Assert that the original [`Signature`] was already in low-S form.
    ///
    /// This method mirrors the functionality of [`stacks-core`][stacks-core].
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if:
    ///
    /// - The recovered [`PublicKey`] does not match the provided key.
    /// - The original signature was not in low-S form.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::ecdsa::Error;
    /// use stacks_primitives::ecdsa::PrivateKey;
    /// use stacks_primitives::ecdsa::PublicKey;
    /// use stacks_primitives::hash::Sha256;
    ///
    /// let pk = PrivateKey::random();
    /// let message = Sha256::compute(b"usque ad finem");
    ///
    /// let signature = pk.sign(message)?;
    /// let key = PublicKey::recover(message, signature)?;
    /// key.verify(message, signature)?;
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// [stacks-core]: https://github.com/stacks-network/stacks-core/blob/master/stacks-common/src/util/secp256k1.rs#L215
    pub fn verify(&self, prehash: H256, sig: Signature) -> Result<(), Error> {
        // Normalize the signature to low-S form
        let normalized = sig.normalize_s();

        // Recover the public key
        let key = Self::recover(prehash, sig)?;

        // Ensure that the recovered key maches the expected key
        if self != &key {
            return Err(Error::new());
        }

        // Ensure that the original signature was already in low-S form
        if sig != normalized {
            return Err(Error::new());
        }

        Ok(())
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PublicKey({:x})", self.to_bytes(true))
    }
}

impl fmt::LowerHex for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.to_bytes(true))
    }
}

impl fmt::UpperHex for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.to_bytes(true))
    }
}

impl str::FromStr for PublicKey {
    type Err = Error;

    /// Creates a [`PublicKey`] from a string.
    ///
    /// This method delegates to [`hex::FromHex`].
    #[inline]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        hex::FromHex::from_hex(str)
    }
}

impl hex::FromHex for PublicKey {
    type Error = Error;

    /// Creates a [`PublicKey`] from a hex-encoded byte sequence.
    ///
    /// Expects a key encoded as a prefixed or unprefixed hex string.
    #[inline]
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        let bytes = hex::decode(hex).map_err(|_| Error::new())?;
        Self::from_bytes(&bytes)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for PublicKey {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if ser.is_human_readable() {
            let bytes = self.to_bytes(true);
            ser.serialize_str(&hex::encode_prefixed(bytes))
        } else {
            let bytes = self.to_bytes(true);
            ser.serialize_bytes(&bytes)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PublicKey {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if de.is_human_readable() {
            use crate::serde::FromHexVisitor;
            de.deserialize_str(FromHexVisitor {
                __msg: format_args!("a hex-encoded 33 or 65 byte public key"),
                __type: marker::PhantomData,
            })
        } else {
            use crate::serde::TryFromVisitor;
            de.deserialize_bytes(TryFromVisitor {
                __msg: format_args!("a 33 or 65 byte sequence of a public key"),
                __type: marker::PhantomData::<(Self, &[u8])>,
            })
        }
    }
}

impl TryFrom<&str> for PublicKey {
    type Error = Error;

    #[inline]
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        <PublicKey as hex::FromHex>::from_hex(str)
    }
}

impl TryFrom<String> for PublicKey {
    type Error = Error;

    #[inline]
    fn try_from(str: String) -> Result<Self, Self::Error> {
        <PublicKey as hex::FromHex>::from_hex(str)
    }
}

impl TryFrom<&[u8]> for PublicKey {
    type Error = Error;

    #[inline]
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<Bytes> for PublicKey {
    type Error = Error;

    #[inline]
    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        Self::from_bytes(&bytes)
    }
}

impl TryFrom<[u8; 33]> for PublicKey {
    type Error = Error;

    #[inline]
    fn try_from(bytes: [u8; 33]) -> Result<Self, Self::Error> {
        Self::from_bytes(&bytes)
    }
}

impl TryFrom<[u8; 65]> for PublicKey {
    type Error = Error;

    #[inline]
    fn try_from(bytes: [u8; 65]) -> Result<Self, Self::Error> {
        Self::from_bytes(&bytes)
    }
}

impl TryFrom<FixedBytes<33>> for PublicKey {
    type Error = Error;

    #[inline]
    fn try_from(bytes: FixedBytes<33>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.raw())
    }
}

impl TryFrom<FixedBytes<65>> for PublicKey {
    type Error = Error;

    #[inline]
    fn try_from(bytes: FixedBytes<65>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.raw())
    }
}

impl TryFrom<Box<[u8]>> for PublicKey {
    type Error = Error;

    #[inline]
    fn try_from(bytes: Box<[u8]>) -> Result<Self, Self::Error> {
        Self::from_bytes(bytes.as_ref())
    }
}

impl TryFrom<Vec<u8>> for PublicKey {
    type Error = Error;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_bytes(bytes.as_slice())
    }
}

/// A fixed-size recoverable ECDSA signature.
///
/// # Layout
///
/// The memory layout of [`Signature`] is a struct with three fields:
///
/// - `r`: A 32-byte value representing the 'r' component.
/// - `s`: A 32-byte value representing the 's' component.
/// - `v`: A 1-byte value representing the recovery ID.
///
/// ```rust,no_format
/// # use core::mem::align_of;
/// # use core::mem::size_of;
/// # use stacks_primitives::ecdsa::Signature;
/// # use stacks_primitives::bytes::B256;
/// # use stacks_primitives::bytes::B8;
/// assert_eq!(size_of::<Signature>(), size_of::<B256>() * 2 + size_of::<B8>());
/// assert_eq!(align_of::<Signature>(), 1);
/// ```
///
/// # Format
///
/// [`Signature`] implements [`fmt::LowerHex`], and [`fmt::UpperHex`] to format
/// itself as a `0x`-prefixed hex string of its encoded bytes:
///
/// ```rust,no_run
/// # use stacks_primitives::ecdsa::Signature;
/// # use stacks_primitives::hex::FromHex;
/// let hex = "0x00354445a1dc98a1bd27984dbe69979a...";
/// let signature = Signature::from_hex(hex);
/// ```
///
/// # Hex
///
/// Hex-encoded strings can be parsed into [`Signature`] via [`hex::FromHex`]:
///
/// ```rust,no_run
/// # use stacks_primitives::ecdsa::Signature;
/// # use stacks_primitives::hex::FromHex;
/// let hex = "0x00354445a1dc98a1bd27984dbe69979a...";
/// let signature = Signature::from_hex(hex);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Signature {
    /// The 32-byte 'r' component.
    r: B256,
    /// The 32-byte 's' component.
    s: B256,
    /// The 1-byte 'v' component.
    v: B8,
}

impl Signature {
    /// Creates a new [`Signature`] from 'r', 's', and 'v' components.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stacks_primitives::bytes::B256;
    /// use stacks_primitives::bytes::B8;
    /// use stacks_primitives::ecdsa::Signature;
    ///
    /// let (r, s, v) = (B256::zero(), B256::zero(), B8::zero());
    /// let signature = Signature::new(r, s, v);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(r: B256, s: B256, v: B8) -> Self {
        Self { r, s, v }
    }

    /// Returns the 'r' component of the [`Signature`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stacks_primitives::bytes::B256;
    /// use stacks_primitives::ecdsa::Signature;
    ///
    /// let signature = Signature::zero();
    /// assert_eq!(signature.r(), B256::zero());
    /// ```
    #[inline]
    #[must_use]
    pub const fn r(&self) -> B256 {
        self.r
    }

    /// Returns the 's' component of the [`Signature`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stacks_primitives::bytes::B256;
    /// use stacks_primitives::ecdsa::Signature;
    ///
    /// let signature = Signature::zero();
    /// assert_eq!(signature.s(), B256::zero());
    /// ```
    #[inline]
    #[must_use]
    pub const fn s(&self) -> B256 {
        self.s
    }

    /// Returns the 'v' component of the [`Signature`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stacks_primitives::bytes::B8;
    /// use stacks_primitives::ecdsa::Signature;
    ///
    /// let signature = Signature::zero();
    /// assert_eq!(signature.v(), B8::zero());
    /// ```
    #[inline]
    #[must_use]
    pub const fn v(&self) -> B8 {
        self.v
    }

    /// Creates a new [`Signature`] with all zeroed components.
    ///
    /// # Notes
    ///
    /// This method produces a signature that is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stacks_primitives::bytes::B256;
    /// use stacks_primitives::bytes::B8;
    /// use stacks_primitives::ecdsa::Signature;
    ///
    /// let (r, s, v) = (B256::zero(), B256::zero(), B8::zero());
    /// let signature = Signature::new(r, s, v);
    /// assert_eq!(signature, Signature::zero());
    /// ```
    #[inline]
    #[must_use]
    pub const fn zero() -> Self {
        let (r, s, v) = (B256::zero(), B256::zero(), B8::zero());
        Self { r, s, v }
    }

    /// Creates a new [`Signature`] from a byte slice.
    ///
    /// # Notes
    ///
    /// This method expects a 65-byte array with the following structure:
    ///
    /// - The first byte is the recovery ID ('v').
    /// - The next 32 bytes are the 'r' component in big-endian format.
    /// - The next 32 bytes are the 's' component in big-endian format.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the slice is not exactly 65 bytes long.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::ecdsa::Error;
    /// use stacks_primitives::bytes::FixedBytes;
    /// use stacks_primitives::ecdsa::Signature;
    ///
    /// let bytes = [0u8; 65];
    /// let signature = Signature::from_slice(&bytes).unwrap();
    /// assert_eq!(signature, Signature::zero());
    /// ```
    #[inline]
    pub fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        let bytes = FixedBytes::try_from(bytes).map_err(|_| Error::new())?;
        Ok(Self::from_bytes(bytes))
    }

    /// Creates a new [`Signature`] from a 65-byte fixed array.
    ///
    /// # Notes
    ///
    /// This method expects a 65-byte array with the following structure:
    ///
    /// - The first byte is the recovery ID ('v').
    /// - The next 32 bytes are the 'r' component in big-endian format.
    /// - The next 32 bytes are the 's' component in big-endian format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stacks_primitives::bytes::FixedBytes;
    /// use stacks_primitives::ecdsa::Signature;
    ///
    /// let bytes = FixedBytes::<65>::zero();
    /// let signature = Signature::from_bytes(bytes);
    /// assert_eq!(signature, Signature::zero());
    /// ```
    #[inline]
    #[must_use]
    pub fn from_bytes(bytes: FixedBytes<65>) -> Self {
        let (v, bytes) = bytes.split::<1, 64>();
        let (r, s) = bytes.split::<32, 32>();
        Self { r, s, v }
    }

    /// Converts the [`Signature`] to a 65-byte fixed array.
    ///
    /// # Notes
    ///
    /// This method produces a 65-byte array with the following structure:
    ///
    /// - The first byte is the recovery ID ('v').
    /// - The next 32 bytes are the 'r' component in big-endian format.
    /// - The next 32 bytes are the 's' component in big-endian format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stacks_primitives::bytes::FixedBytes;
    /// use stacks_primitives::ecdsa::Signature;
    ///
    /// let signature = Signature::zero();
    /// let bytes = signature.to_bytes();
    /// assert_eq!(bytes.len(), 65);
    /// ```
    #[inline]
    #[must_use]
    pub const fn to_bytes(&self) -> FixedBytes<65> {
        self.v.concat::<33, 32>(self.r).concat(self.s)
    }

    /// Creates a [`Signature`] from [`k256`] primitives.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::ecdsa::Error;
    /// use stacks_primitives::ecdsa::k256;
    /// use stacks_primitives::ecdsa::PrivateKey;
    /// use stacks_primitives::ecdsa::Signature;
    /// use stacks_primitives::hash::Sha256;
    ///
    /// let pk = k256::SigningKey::random(&mut rand_core::OsRng);
    /// let message = Sha256::compute(b"usque ad finem");
    ///
    /// let (s, r) = pk.sign_prehash_recoverable(&message[..])?;
    /// let signature = Signature::from_k256(s, r);
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    #[must_use]
    pub fn from_k256(sig: k256::Signature, id: k256::RecoveryId) -> Self {
        let r = B256::new(sig.r().to_bytes().into());
        let s = B256::new(sig.s().to_bytes().into());
        let v = B8::from(id.to_byte());
        Self { r, s, v }
    }

    /// Converts the [`Signature`] to [`k256`] primitives.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if:
    ///
    /// - The 'r' component is not a valid scalar.
    /// - The 's' component is not a valid scalar.
    /// - The 'v' component is not a valid [`k256::RecoveryId`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::ecdsa::Error;
    /// use stacks_primitives::ecdsa::PrivateKey;
    /// use stacks_primitives::ecdsa::Signature;
    /// use stacks_primitives::hash::Sha256;
    ///
    /// let pk = PrivateKey::random();
    /// let message = Sha256::compute(b"usque ad finem");
    ///
    /// let signature = pk.sign(message)?;
    /// let (s, r) = signature.to_k256()?;
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn to_k256(self) -> Result<(k256::Signature, k256::RecoveryId), Error> {
        let s = k256::Signature::from_scalars(self.r.raw(), self.s.raw())?;
        let r = k256::RecoveryId::try_from(u8::from(self.v))?;
        Ok((s, r))
    }

    /// Normalizes the 's' component of the [`Signature`].
    ///
    /// # Notes
    ///
    /// Normalization ensures that 's' is in the lower half of the curve order.
    ///
    /// The process involves the following steps:
    ///
    /// - Setting the 's' component to `Secp256k1::ORDER - s`.
    /// - Flipping the 'v' component by `XORing` with 1.
    /// - Returns the original signature if 's' is already normal.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::ecdsa::Error;
    /// use stacks_primitives::ecdsa::PrivateKey;
    /// use stacks_primitives::ecdsa::Signature;
    /// use stacks_primitives::hash::Sha256;
    ///
    /// let pk = PrivateKey::random();
    /// let message = Sha256::compute(b"usque ad finem");
    ///
    /// let signature = pk.sign(message)?;
    /// let normalized = signature.normalize_s();
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// For more details, see the [`BIP-0062`][BIP-0062].
    ///
    /// [BIP-0062]: https://github.com/bitcoin/bips/blob/master/bip-0062.mediawiki
    #[inline]
    #[must_use]
    pub fn normalize_s(self) -> Self {
        use k256::Curve;
        use k256::Encoding;
        use k256::Secp256k1;

        // Convert the 's' component to a 'U256'
        let s = k256::U256::from_be_bytes(self.s.raw());

        // Check if 's' exceeds half of the secp256k1 order (n / 2)
        if s > Secp256k1::ORDER >> 1 {
            // Normalize 's' by subtracting it from the curve order (n - s)
            let s = B256::from(s.neg_mod(&Secp256k1::ORDER).to_be_bytes());

            // Flip 'v' to reflect the normalization
            let v = B8::from(u8::from(self.v) ^ 1);

            // Construct the signature with the normalized 's'
            Self { s, v, ..self }
        } else {
            // If 's' is already ≤ (n / 2), return the signature unchanged
            self
        }
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Signature")
            .field("r", &format_args!("{:x}", self.r))
            .field("s", &format_args!("{:x}", self.s))
            .field("v", &format_args!("{:x}", self.v))
            .finish()
    }
}

impl fmt::LowerHex for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.to_bytes())
    }
}

impl fmt::UpperHex for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.to_bytes())
    }
}

impl str::FromStr for Signature {
    type Err = Error;

    /// Creates a [`Signature`] from a string.
    ///
    /// This method delegates to [`hex::FromHex`].
    #[inline]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        hex::FromHex::from_hex(str)
    }
}

impl hex::FromHex for Signature {
    type Error = Error;

    /// Creates a [`Signature`] from a hex-encoded byte sequence.
    ///
    /// Expects a 65-byte sig encoded as a prefixed or unprefixed hex string.
    #[inline]
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        let decoded = hex::decode(hex).map_err(|_| Error::new())?;
        let mut bytes = FixedBytes::<65>::zero();
        bytes.copy_from_slice(&decoded);
        Ok(Self::from_bytes(bytes))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Signature {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bytes = self.to_bytes();
        bytes.serialize(ser)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Signature {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if de.is_human_readable() {
            use crate::serde::FromHexVisitor;
            de.deserialize_str(FromHexVisitor {
                __msg: format_args!("a hex-encoded 65-byte ECDSA signature"),
                __type: marker::PhantomData,
            })
        } else {
            use crate::serde::TryFromVisitor;
            de.deserialize_bytes(TryFromVisitor {
                __msg: format_args!("a 65-byte sequence of an ECDSA signature"),
                __type: marker::PhantomData::<(Self, &[u8])>,
            })
        }
    }
}

impl From<[u8; 65]> for Signature {
    #[inline]
    fn from(bytes: [u8; 65]) -> Self {
        Self::from_bytes(bytes.into())
    }
}

impl From<FixedBytes<65>> for Signature {
    #[inline]
    fn from(bytes: FixedBytes<65>) -> Self {
        Self::from(bytes.raw())
    }
}

impl TryFrom<&str> for Signature {
    type Error = Error;

    #[inline]
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        <Signature as hex::FromHex>::from_hex(str)
    }
}

impl TryFrom<String> for Signature {
    type Error = Error;

    #[inline]
    fn try_from(str: String) -> Result<Self, Self::Error> {
        <Signature as hex::FromHex>::from_hex(str)
    }
}

impl TryFrom<&[u8]> for Signature {
    type Error = Error;

    #[inline]
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::from_slice(bytes)
    }
}

impl TryFrom<Bytes> for Signature {
    type Error = Error;

    #[inline]
    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        Self::from_slice(&bytes)
    }
}

impl TryFrom<Box<[u8]>> for Signature {
    type Error = Error;

    #[inline]
    fn try_from(bytes: Box<[u8]>) -> Result<Self, Self::Error> {
        Self::from_slice(bytes.as_ref())
    }
}

impl TryFrom<Vec<u8>> for Signature {
    type Error = Error;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_slice(bytes.as_slice())
    }
}
