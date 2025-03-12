use args::parse_args;
use hex;

pub fn main() {
    let (prove_main, verify_main) = guest::build_entrypoint();
    let (prove_main_vec, verify_main_vec) = guest::build_entrypoint_vec();

    let mut args = parse_args().unwrap();
    args.remove(0).value;
    for arg in args {
        match arg.kind.as_str() {
            "uint32" => {
                let input_data = u32::from_str_radix(&arg.value, 10).unwrap();
                let (output, proof) = prove_main(input_data);
                let is_valid = verify_main(proof);

                println!("output: {}", output);
                println!("valid: {}", is_valid);
            }
            "bytearray" => {
                let value = if arg.value.starts_with("0x") {
                    &arg.value[2..]
                } else {
                    &arg.value
                };
                let bytes = match hex::decode(value) {
                    Ok(b) => {b},
                    Err(e) => {
                        eprintln!("Failed to decode hex string: {}", e);
                        return;
                    }
                };
                let (output, proof) = prove_main_vec(&bytes);
                let is_valid = verify_main_vec(proof);

                println!("output: {:?}", output);
                println!("valid: {}", is_valid);
            }
            _ => {
                eprintln!("Unknown argument kind: {}", arg.kind);
            }
        }
    }

    // let (output, proof) = prove_main(input_data);
    // let is_valid = verify_main(proof);

    // println!("output: {}", output);
    // println!("valid: {}", is_valid);
}
