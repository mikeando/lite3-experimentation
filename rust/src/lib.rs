//! Core data structures for the Rust implementation of Lite3.

// Constants from the C implementation, ensuring binary compatibility.
pub const LITE3_NODE_SIZE: usize = 96;
pub const LITE3_NODE_KEY_COUNT_MAX: usize = 7;
pub const LITE3_NODE_KEY_COUNT_MIN: usize = LITE3_NODE_KEY_COUNT_MAX / 2;

pub const LITE3_NODE_TYPE_SHIFT: u32 = 0;
pub const LITE3_NODE_TYPE_MASK: u32 = (1 << 8) - 1; // 8 LSB

pub const LITE3_NODE_GEN_SHIFT: u32 = 8;
pub const LITE3_NODE_GEN_MASK: u32 = !((1 << 8) - 1); // 24 MSB

pub const LITE3_NODE_KEY_COUNT_SHIFT: u32 = 0;
pub const LITE3_NODE_KEY_COUNT_MASK: u32 = (1 << 3) - 1; // 3 LSB for key_count: 0-7

pub const LITE3_NODE_SIZE_SHIFT: u32 = 6;
pub const LITE3_NODE_SIZE_MASK: u32 = !((1 << 6) - 1); // 26 MSB


/// Represents the different types of values that can be stored in a Lite3 buffer.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lite3Type {
    Null,
    Bool,
    I64,
    F64,
    Bytes,
    String,
    Object,
    Array,
    Invalid,
    Count,
}

/// Represents a B-tree node in the Lite3 buffer.
/// The `#[repr(C)]` attribute ensures that the memory layout of this struct
/// is the same as the C struct, which is essential for binary compatibility.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Node {
    /// upper 24 bits: gen, lower 8 bits: lite3_type
    pub gen_type: u32,
    pub hashes: [u32; LITE3_NODE_KEY_COUNT_MAX],
    /// upper 26 bits: size, lower 6 bits: key_count
    pub size_kc: u32,
    pub kv_ofs: [u32; LITE3_NODE_KEY_COUNT_MAX],
    pub child_ofs: [u32; LITE3_NODE_KEY_COUNT_MAX + 1],
}

/// Represents a value in the Lite3 buffer.
/// The `val` field is a zero-sized array, which is Rust's equivalent of a
/// flexible array member in C. This allows us to have a dynamically sized
/// struct that can be laid out in memory correctly.
#[repr(C)]
#[derive(Debug)]
pub struct Lite3Val {
    pub type_: u8,
    pub val: [u8; 0],
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_node_size() {
        assert_eq!(mem::size_of::<Node>(), LITE3_NODE_SIZE);
    }
}
