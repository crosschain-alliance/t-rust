extern crate userscrate;

use args::parse_args;
use t_rust::{read_public_outputs_vecs, write_input, write_input_slice};

pub fn main() {
    let args = parse_args().unwrap();
    for arg in args {
        match arg.kind.as_str() {
            "uint32" => {
                write_input::<u32>(&u32::from_str_radix(&arg.value, 10).unwrap());
            }
            "bytearray" => {
                write_input_slice(&hex::decode(&arg.value).unwrap());
            }
            _ => {
                eprintln!("Unknown argument kind: {}", arg.kind);
            }
        }
    }

    userscrate::main();

    let outputs = read_public_outputs_vecs();
    println!("{:?}", outputs);

    // TODO: handle outputs
}
