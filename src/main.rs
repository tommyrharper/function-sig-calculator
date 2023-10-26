use sha3::{Digest, Keccak256};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let function_name = "getAnotherValue()"; // should output 0x25d805cf
    let function_name = &args[1]; // should output 0x25d805cf
    let hashed_data = Keccak256::digest(function_name);

    println!("{:#02x}{:02x}{:02x}{:02x}", hashed_data[0], hashed_data[1], hashed_data[2], hashed_data[3]);
}
