use args::parse_args;
use sp1_sdk::{ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
///
/// This file is generated by running `cargo prove build` inside the `program` directory.
pub const PROGRAM_ELF: &[u8] = include_bytes!("../../../builder/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    sp1_sdk::utils::setup_logger();
    let client = ProverClient::new();
    let (pk, vk) = client.setup(PROGRAM_ELF);
    let mut stdin = SP1Stdin::new();

    let args = parse_args().unwrap();
    for arg in args {
        match arg.kind.as_str() {
            "uint32" => {
                stdin.write::<u32>(&u32::from_str_radix(&arg.value, 10).unwrap());
            }
            "bytearray" => {
                stdin.write_slice(&hex::decode(&arg.value).unwrap());
            }
            _ => {
                eprintln!("Unknown argument kind: {}", arg.kind);
            }
        }
    }

    let proof = client.prove(&pk, stdin).run().unwrap();
    client.verify(&proof, &vk).unwrap();
    proof
        .save("/tmp/proofs/proof.bin")
        .expect("saving proof failed");
}
