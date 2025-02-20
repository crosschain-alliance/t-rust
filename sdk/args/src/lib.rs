use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};
use std::{fs::File, path::Path};

const TRUST_ARGS_FILENAME: &str = "/tmp/trust.rargs";

#[derive(Debug, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub value: String,
    pub kind: String,
}

/// Reads and parses arguments from the trust arguments file
///
/// # Returns
/// - `Ok(Vec<Argument>)` containing the parsed arguments
/// - `Err(Error)` if file operations or parsing fails
///
/// # Example
/// ```rust
/// use args::parse_args;
///
/// fn main() -> anyhow::Result<()> {
///     let args = parse_args()?;
///     for arg in args {
///         println!("Found argument: {:?}", arg);
///     }
///     Ok(())
/// }
/// ```
pub fn parse_args() -> Result<Vec<Argument>> {
    let args_path = Path::new(TRUST_ARGS_FILENAME);
    
    // Open and read the arguments file
    let args_file = File::open(args_path)
        .with_context(|| format!("Failed to open arguments file at {}", args_path.display()))?;

    // Parse the CBOR content into our Argument structure
    let args: Vec<Argument> = serde_cbor::from_reader(args_file)
        .context("Failed to parse arguments file content as CBOR")?;

    // Validate that we have at least one argument
    if args.is_empty() {
        anyhow::bail!("No arguments found in arguments file");
    }

    Ok(args)
}
