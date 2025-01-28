#![cfg_attr(feature = "jolt", no_std)]

#[cfg(feature = "local")]
include!("backends/local.rs");

#[cfg(feature = "sp1")]
include!("backends/sp1.rs");

#[cfg(feature = "risc0")]
include!("backends/risc0.rs");

#[cfg(feature = "jolt")]
include!("backends/jolt.rs");

#[cfg(not(any(feature = "local", feature = "sp1", feature = "risc0", feature = "jolt")))]
compile_error!(
    "No specific backend is enabled. Please enable either the `local`,`sp1`,`risc0` or `jolt` feature."
);
