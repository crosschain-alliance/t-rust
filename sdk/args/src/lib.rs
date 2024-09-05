use serde_derive::{Deserialize, Serialize};
use std::fs::File;

const TRUST_ARGS_FILENAME: &str = "/tmp/trust.rargs";

#[derive(Debug, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub value: String,
    pub kind: String,
}

pub fn parse_args() -> Result<Vec<Argument>, ()> {
    let args_file = File::open(TRUST_ARGS_FILENAME).unwrap();
    let args: Vec<Argument> = serde_cbor::from_reader(args_file).unwrap();
    Ok(args)
}
