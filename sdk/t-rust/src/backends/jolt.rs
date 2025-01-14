static mut STATE: u32 = 0;

/// Fetch the value from the shared state.
pub fn read<T>() -> T
where
    T: From<u32>, // Ensure T can be constructed from a u32
{
    unsafe { T::from(STATE) }
}

/// Set the value into the shared state.
pub fn commit<T>(value: &T)
where
    T: Into<u32> + Copy, // Allow conversion from any type to u32 and ensure T is Copy
{
    unsafe {
        STATE = (*value).into(); // Dereference the reference and convert
    }
}