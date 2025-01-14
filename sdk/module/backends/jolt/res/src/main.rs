use args::parse_args;

pub fn main() {
    let (prove_main, verify_main) = guest::build_entrypoint();

    let mut input_data: u32 = 0;
    let args = parse_args().unwrap();
    for arg in args {
        match arg.kind.as_str() {
            "uint32" => {
                input_data = u32::from_str_radix(&arg.value, 10).unwrap();
                // input_data.extend(&to_vec(&value).unwrap());
            }
            // "bytearray" => {
            //     let bytes = hex::decode(&arg.value).unwrap();
            //     input_data.extend(&to_vec(&bytes).unwrap());
            // }
            _ => {
                eprintln!("Unknown argument kind: {}", arg.kind);
            }
        }
    }

    let (output, proof) = prove_main(input_data);
    let is_valid = verify_main(proof);

    println!("output: {}", output);
    println!("valid: {}", is_valid);
}
