//! byte_utils.rs
//! Provides functions to safely and efficiently convert raw byte arrays into basic numeric types.
//! Ensures correct endianness handling, which is crucial for parsing binary market data formats.
//!
//! # Key Points
//! - Market data messages often come in big-endian or little-endian formats.
//! - `byte_utils` helps ensure consistent interpretation of numeric fields.
//!
//! # Example
//! ```
//! use common::byte_utils; // Import the module
//!
//! let raw = &[0x01, 0x00, 0x00, 0x00];
//! let val = byte_utils::le_to_u32(raw);
//! assert_eq!(val, 1);
//! ```

use std::convert::TryInto;

/// Converts a little-endian byte slice into a `u32`.
/// Panics if slice length is not 4.
///
/// # Examples
/// ```
/// use common::byte_utils;
///
/// let raw = &[0x01, 0x00, 0x00, 0x00];
/// let val = byte_utils::le_to_u32(raw);
/// assert_eq!(val, 1);
/// ```
pub fn le_to_u32(data: &[u8]) -> u32 {
    let bytes: [u8; 4] = data.try_into().expect("Invalid slice length for u32");
    u32::from_le_bytes(bytes)
}

/// Converts a little-endian byte slice into a `u64`.
/// Panics if slice length is not 8.
///
/// # Examples
/// ```
/// use common::byte_utils;
///
/// let raw = &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
/// let val = byte_utils::le_to_u64(raw);
/// assert_eq!(val, 1);
/// ```
pub fn le_to_u64(data: &[u8]) -> u64 {
    let bytes: [u8; 8] = data.try_into().expect("Invalid slice length for u64");
    u64::from_le_bytes(bytes)
}

/// Converts a little-endian byte slice into an `f64`.
/// Panics if slice length is not 8.
///
/// # Examples
/// ```
/// use common::byte_utils;
///
/// let raw = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F]; // Little-endian representation of 1.0
/// let val = byte_utils::le_to_f64(raw);
/// assert_eq!(val, 1.0);
/// ```
pub fn le_to_f64(data: &[u8]) -> f64 {
    let bits = le_to_u64(data);
    f64::from_bits(bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_le_to_u32() {
        let raw = &[0x01, 0x00, 0x00, 0x00];
        assert_eq!(le_to_u32(raw), 1);
    }

    #[test]
    #[should_panic(expected = "Invalid slice length for u32")]
    fn test_le_to_u32_invalid_length() {
        let raw = &[0x01, 0x00, 0x00]; // Only 3 bytes
        le_to_u32(raw);
    }

    #[test]
    fn test_le_to_u64() {
        let raw = &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(le_to_u64(raw), 1);
    }

    #[test]
    #[should_panic(expected = "Invalid slice length for u64")]
    fn test_le_to_u64_invalid_length() {
        let raw = &[0x01, 0x00, 0x00, 0x00]; // Only 4 bytes
        le_to_u64(raw);
    }

    #[test]
    fn test_le_to_f64() {
        let raw = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F]; // Little-endian representation of 1.0
        assert_eq!(le_to_f64(raw), 1.0);
    }
}
