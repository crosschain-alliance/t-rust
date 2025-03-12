use sp1_sdk::{HashableKey, ProverClient};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
/// This file is generated by running `cargo prove build` inside the `program` directory.
pub const PROGRAM_ELF: &[u8] = include_bytes!("../../../builder/target/elf-compilation/riscv32im-succinct-zkvm-elf/release/sp1-program");

fn main() {
    let client = ProverClient::from_env();
    let (_, vk) = client.setup(PROGRAM_ELF);
    println!("{}", vk.bytes32().to_string());
}
