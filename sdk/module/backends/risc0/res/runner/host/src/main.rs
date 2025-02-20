use anyhow::{Context, Result};
use std::{fs, path::PathBuf};
use args::parse_args;
use methods::{PROGRAM_ELF, PROGRAM_ID};
use risc0_zkvm::{default_prover, default_executor, ExecutorEnv, ExecutorEnvBuilder, Receipt};
use hex;

const PROOF_OUTPUT_PATH: &str = "/tmp/proofs/risc0Proof.bin";
const INPUT_FILE_PATH: &str = "/risc0_target/input.file";

struct ProverConfig<'a> {
    mode: String,
    env: ExecutorEnv<'a>,
}

impl<'a> ProverConfig<'a> {
    fn new(args: Vec<args::Argument>) -> Result<Self> {
        let mut args = args;
        let mode = args.remove(0).value;
        let mut ex_env = ExecutorEnv::builder();
        
        for arg in args {
            process_argument(&mut ex_env, &arg)
                .with_context(|| format!("Failed to process argument: {:?}", arg))?;
        }

        let env = ex_env.build()
            .context("Failed to build executor environment")?;
        
        Ok(Self { mode, env })
    }
}

fn process_argument(ex_env: &mut ExecutorEnvBuilder, arg: &args::Argument) -> Result<()> {
    match arg.kind.as_str() {
        "uint32" => {
            let value = u32::from_str_radix(&arg.value, 10)
                .with_context(|| format!("Failed to parse uint32: {}", arg.value))?;
            ex_env.write(&value)
                .context("Failed to write uint32 to environment")?;
        }
        "bytearray" => {
            let value = if arg.value.starts_with("0x") {
                &arg.value[2..]
            } else {
                &arg.value
            };
            let bytes = hex::decode(value)
                .with_context(|| format!("Failed to decode hex string: {}", value))?;
            ex_env.write(&bytes)
                .context("Failed to write bytes to environment")?;
        }
        "file" => {
            let buffer = fs::read(INPUT_FILE_PATH)
                .context("Failed to read input file")?;
            ex_env.write(&buffer)
                .context("Failed to write file contents to environment")?;
        }
        unknown => {
            anyhow::bail!("Unknown argument kind: {}", unknown);
        }
    }
    Ok(())
}

fn execute_program(env: ExecutorEnv) -> Result<()> {
    let exec = default_executor();
    let session = exec.execute(env, PROGRAM_ELF)
        .context("Program execution failed")?;
    println!("Execution Output: {:?}", session.journal);
    Ok(())
}

fn prove(env: ExecutorEnv) -> Result<()> {
    let prover = default_prover();

    let prove_info = prover.prove(env, PROGRAM_ELF)
        .context("Proof generation failed")?;
    
    let output_path = PathBuf::from(PROOF_OUTPUT_PATH);
    let receipt_data = bincode::serialize(&prove_info.receipt)
        .context("Failed to serialize receipt")?;
    
    fs::write(&output_path, receipt_data)
        .with_context(|| format!("Failed to write proof to {}", output_path.display()))?;
    
    println!("Proof saved to: {}", PROOF_OUTPUT_PATH);
    Ok(())
}

fn verify() -> Result<()> {
    let receipt_data = fs::read(PathBuf::from(INPUT_FILE_PATH)).context("Failed to load proof")?;
    let receipt: Receipt = bincode::deserialize(&receipt_data).context("Failed to deserialize proof")?;

    receipt
        .verify(PROGRAM_ID)
        .context("Proof verification failed")?;
    
    println!("Proof verified successfully");
    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let config = ProverConfig::new(parse_args()?)?;

    match config.mode.as_str() {
        "run" => execute_program(config.env)?,
        "prove" | "benchmark" => prove(config.env)?,
        "verify" => verify()?,
        mode => anyhow::bail!("Unknown mode: {}", mode),
    }
    
    Ok(())
}