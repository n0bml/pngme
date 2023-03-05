use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

fn is_valid_byte(byte: u8) -> bool {
    if byte.is_ascii_uppercase() || byte.is_ascii_lowercase() {
        true
    } else {
        false
    }
}

/// Byte representation of a PNG version 1.2 Chunk.
/// `ChunkType` provides validation for a PNG version 1.2
/// chunk, also provides construction and conversion for the
/// same type.
///
/// # Structure
///
/// ```ignore
/// bLOb  <-- 32 bit chunk type code represented in text form
/// ||||
/// |||+- Safe-to-copy bit is 1 (lowercase letter; bit 5 is 1)
/// ||+-- Reserved bit is 0     (uppercase letter; bit 5 is 0)
/// |+--- Private bit is 0      (uppercase letter; bit 5 is 0)
/// +---- Ancillary bit is 1    (lowercase letter; bit 5 is 1)
/// ```
/// [Source](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [self.0[0], self.0[1], self.0[2], self.0[3]]
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..self.0.len() {
            if !is_valid_byte(self.0[i]) {
                return false;
            }
        }

        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        self.0[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.0[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.0[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.0[3].is_ascii_lowercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self([value[0], value[1], value[2], value[3]]))
    }
}

impl FromStr for ChunkType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("Invalid PNG chunk type length!".into());
        }

        let bytes = s.bytes();
        if !bytes.clone().all(is_valid_byte) {
            return Err("Invalid PNG chunk type byte!".into());
        }

        let slice = s.bytes().collect::<Vec<u8>>();
        Ok(Self([slice[0], slice[1], slice[2], slice[3]]))
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
