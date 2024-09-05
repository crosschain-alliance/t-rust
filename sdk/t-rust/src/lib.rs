#[cfg(feature = "local")]
include!("backends/local.rs");

// This will act as a fallback if no feature is enabled.
#[cfg(not(any(
    feature = "local"
)))]
panic!("No specific backend is enabled");
