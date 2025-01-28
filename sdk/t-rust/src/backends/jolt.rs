extern crate alloc;
use alloc::vec::Vec;

static mut STATE: [u8; 32] = [0; 32];
static mut VEC_LEN: usize = 0;

pub fn read<T: Handle>() -> T
{
    unsafe { T::from_bytes(&STATE) }
}

pub fn read_vec<T: Handle>() -> T
{
    unsafe { T::from_bytes(&STATE) }
}

pub fn commit<T: Handle>(value: &T)
{
    unsafe {
        STATE = value.into_bytes();
    }
}

// Trait to support conversion
trait Handle: Sized {
    fn into_bytes(&self) -> [u8; 32];
    fn from_bytes(bytes: &[u8; 32]) -> Self;
}

impl Handle for u32 {
    fn into_bytes(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        bytes[0..4].copy_from_slice(&self.to_le_bytes());
        bytes
    }

    fn from_bytes(bytes: &[u8; 32]) -> Self {
        u32::from_le_bytes(bytes[0..4].try_into().expect("Invalid byte length"))
    }
}

impl Handle for Vec<u8> {
    fn into_bytes(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        unsafe { 
            VEC_LEN = self.len().min(32);
            bytes[0..VEC_LEN].copy_from_slice(&self[0..VEC_LEN]); 
        }
        bytes
    }

    fn from_bytes(bytes: &[u8; 32]) -> Self {
        unsafe { bytes[0..VEC_LEN].to_vec() }
    }
}

impl Handle for [u8; 32] {
    fn into_bytes(&self) -> [u8; 32] {
        *self
    }

    fn from_bytes(bytes: &[u8; 32]) -> Self {
        *bytes
    }
}
