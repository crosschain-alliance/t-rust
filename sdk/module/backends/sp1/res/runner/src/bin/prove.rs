use anyhow::{Context, Result};
use args::parse_args;
use sp1_sdk::{EnvProver, ProverClient, SP1Stdin, SP1ProofWithPublicValues};
use std::path::Path;
use hex;

const PROGRAM_ELF: &[u8] = include_bytes!("../../../builder/target/elf-compilation/riscv32im-succinct-zkvm-elf/release/sp1-program");
const PROOF_OUTPUT_PATH: &str = "/tmp/proofs/sp1Proof.bin";
const INPUT_FILE_PATH: &str = "/sp1_target/input.file";

struct ProverConfig {
    mode: String,
    stdin: SP1Stdin,
}

impl ProverConfig {
    fn new(args: Vec<args::Argument>) -> Result<Self> {
        let mut args = args;
        let mode = args.remove(0).value;
        let mut stdin = SP1Stdin::new();
        
        for arg in args {
            process_argument(&mut stdin, &arg)?;
        }
        
        Ok(Self { mode, stdin })
    }
}

fn process_argument(stdin: &mut SP1Stdin, arg: &args::Argument) -> Result<()> {
    match arg.kind.as_str() {
        "uint32" => {
            let value = u32::from_str_radix(&arg.value, 10)
                .context("Failed to parse uint32")?;
            stdin.write::<u32>(&value);
        }
        "bytearray" => {
            let bytes = hex::decode(&arg.value)
                .context("Failed to decode hex string")?;
            stdin.write_slice(&bytes);
        }
        "file" => {
            let buffer = std::fs::read(INPUT_FILE_PATH)
                .context("Failed to read input file")?;
            stdin.write_slice(&buffer);
        }
        unknown => {
            anyhow::bail!("Unknown argument type: {}", unknown);
        }
    }
    Ok(())
}

fn execute_program(client: &EnvProver, stdin: &SP1Stdin) -> Result<()> {
    let (output, _report) = client.execute(PROGRAM_ELF, stdin)
        .run()
        .context("Program execution failed")?;
    println!("Output: {:?}", output);
    Ok(())
}

fn prove(client: &EnvProver, stdin: &SP1Stdin) -> Result<()> {
    let (proving_key, _verification_key) = client.setup(PROGRAM_ELF);
    
    let proof = client.prove(&proving_key, stdin)
        .run()
        .context("Proof generation failed")?;
    
    proof.save(Path::new(PROOF_OUTPUT_PATH))
        .context("Failed to save proof")?;
    
    println!("Proof saved to: {}", PROOF_OUTPUT_PATH);
    
    Ok(())
}

fn verify(client: &EnvProver) -> Result<()> {
    let (_proving_key, verification_key) = client.setup(PROGRAM_ELF);
    
    let proof = SP1ProofWithPublicValues::load(Path::new(INPUT_FILE_PATH))
        .context("Failed to save proof")?;

    client.verify(&proof, &verification_key)
        .context("Proof verification failed")?;
    
    println!("Proof verified successfully");
    
    Ok(())
}

fn main() -> Result<()> {
    sp1_sdk::utils::setup_logger();
    let client = ProverClient::from_env();
    
    let args = parse_args().map_err(|_| anyhow::anyhow!("Failed to parse arguments"))?;
    let config = ProverConfig::new(args)?;
    
    match config.mode.as_str() {
        "run" => execute_program(&client, &config.stdin)?,
        "prove" | "benchmark" => prove(&client, &config.stdin)?,
        "verify" => verify(&client)?,
        mode => anyhow::bail!("Unknown mode: {}", mode),
    }
    
    Ok(())
}
