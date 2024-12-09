use sp1_sdk::{HashableKey, ProverClient};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
///
/// This file is generated by running `cargo prove build` inside the `program` directory.
pub const PROGRAM_ELF: &[u8] = include_bytes!("../../../builder/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    //sp1_sdk::utils::setup_logger();
    let client = ProverClient::new();
    let (_, vk) = client.setup(PROGRAM_ELF);
    println!("{}", vk.bytes32().to_string());
}
