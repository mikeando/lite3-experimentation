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

/// Represents the possible errors that can occur when working with the Buffer API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferError {
    /// The provided buffer is too small.
    NoBufs,
    /// An invalid argument was provided.
    Invalid,
    /// The message is malformed.
    BadMsg,
    /// A key was not found.
    NoEnt,
    /// An I/O error occurred.
    Io,
    /// A bad address was encountered.
    Fault,
    /// The message is too long.
    MsgSize,
    /// A value is too large for the defined data type.
    Overflow,
}

/// The result type for Buffer API operations.
pub type BufferResult<T> = Result<T, BufferError>;

/// Provides a low-level, unsafe API for interacting with a Lite3 buffer.
/// This API is designed to be a direct translation of the C implementation.
pub struct BufferApi<'a> {
    buf: &'a mut [u8],
    buflen: &'a mut usize,
}

impl<'a> BufferApi<'a> {
    /// Creates a new `BufferApi` instance.
    pub fn new(buf: &'a mut [u8], buflen: &'a mut usize) -> Self {
        Self { buf, buflen }
    }

    /// Initializes the buffer as an object.
    pub fn init_obj(&mut self) -> BufferResult<()> {
        if self.buf.len() < LITE3_NODE_SIZE {
            return Err(BufferError::NoBufs);
        }
        _lite3_init_impl(self.buf, 0, Lite3Type::Object);
        *self.buflen = LITE3_NODE_SIZE;
        Ok(())
    }

    /// Initializes the buffer as an array.
    pub fn init_arr(&mut self) -> BufferResult<()> {
        if self.buf.len() < LITE3_NODE_SIZE {
            return Err(BufferError::NoBufs);
        }
        _lite3_init_impl(self.buf, 0, Lite3Type::Array);
        *self.buflen = LITE3_NODE_SIZE;
        Ok(())
    }

    /// Set null in object
    pub fn set_null(&mut self, ofs: usize, key: &str) -> BufferResult<()> {
        let key_data = lite3_get_key_data(key);
        // for LITE3_TYPE_NULL, the value length is 0.
        let val = self._lite3_set_impl(ofs, key, key_data, 0)?;
        unsafe {
            (*val).type_ = Lite3Type::Null as u8;
        }
        Ok(())
    }

    #[allow(unused_variables)]
    fn _lite3_set_impl(
        &mut self,
        ofs: usize,
        key: &str,
        key_data: Lite3KeyData,
        val_len: usize,
    ) -> BufferResult<*mut Lite3Val> {
        // This is a placeholder for the full implementation of the set logic,
        // which is quite complex. For now, we'll just return an error.
        Err(BufferError::Invalid)
    }
}

/// Struct to hold key data.
struct Lite3KeyData {
    hash: u32,
    size: u32,
}

/// Calculates the DJB2 hash of a key.
fn lite3_get_key_data(key: &str) -> Lite3KeyData {
    let mut hash = 5381u32;
    for byte in key.bytes() {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u32);
    }
    Lite3KeyData {
        hash,
        size: key.len() as u32 + 1, // include null terminator
    }
}

fn _lite3_init_impl(buf: &mut [u8], ofs: usize, type_: Lite3Type) {
    let node = unsafe { &mut *(buf.as_mut_ptr().add(ofs) as *mut Node) };
    node.gen_type = type_ as u32 & LITE3_NODE_TYPE_MASK;
    node.size_kc = 0x00;
    node.hashes.iter_mut().for_each(|x| *x = 0);
    node.kv_ofs.iter_mut().for_each(|x| *x = 0);
    node.child_ofs.iter_mut().for_each(|x| *x = 0);
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
