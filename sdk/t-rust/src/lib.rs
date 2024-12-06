#[cfg(feature = "local")]
include!("backends/local.rs");

#[cfg(feature = "sp1")]
include!("backends/sp1.rs");

#[cfg(not(any(feature = "local", feature = "sp1")))]
compile_error!(
    "No specific backend is enabled. Please enable either the `local` or `sp1` feature."
);
