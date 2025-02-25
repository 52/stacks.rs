// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

/// Copies `n` bytes from `src` to `dst`.
///
/// # Panics
///
/// Panics if either:
/// - `src_offset + n > src.len()`
/// - `dst_offset + n > dst.len()`
///
/// # Examples
///
/// ```rust
/// use stacks_primitives::utils;
///
/// let src = [1, 2, 3, 4, 5];
/// let mut dst = [0; 5];
///
/// utils::memcpy(&mut dst, 1, &src, 2, 3);
/// assert_eq!(dst, [0, 3, 4, 5, 0]);
/// ```
#[inline]
#[track_caller]
pub const fn memcpy(
    dst: &mut [u8],
    dst_offset: usize,
    src: &[u8],
    src_offset: usize,
    n: usize,
) {
    assert!(src_offset + n <= src.len(), "Range 'src' exceeds bounds");
    assert!(dst_offset + n <= dst.len(), "Range 'dst' exceeds bounds");

    let mut i = 0;
    while i < n {
        dst[dst_offset + i] = src[src_offset + i];
        i += 1;
    }
}

/// Compares `n` bytes between two byte slices.
///
/// # Panics
///
/// Panics if either:
/// - `a_offset + n > a.len()`
/// - `b_offset + n > b.len()`
///
/// # Examples
///
/// ```rust
/// use stacks_primitives::utils;
///
/// let a = [1, 2, 3, 4, 5];
/// let b = [0, 1, 2, 3, 0];
///
/// assert!(utils::memcmp(&a, 0, &b, 1, 3));
/// assert!(!utils::memcmp(&a, 0, &b, 0, 3));
/// ```
#[inline]
#[track_caller]
pub const fn memcmp(
    a: &[u8],
    a_offset: usize,
    b: &[u8],
    b_offset: usize,
    n: usize,
) -> bool {
    assert!(a_offset + n <= a.len(), "Range 'a' exceeds bounds");
    assert!(b_offset + n <= b.len(), "Range 'b' exceeds bounds");

    let mut i = 0;
    while i < n {
        if a[a_offset + i] != b[b_offset + i] {
            return false;
        }
        i += 1;
    }
    true
}
