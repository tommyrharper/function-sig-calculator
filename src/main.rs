use sha3::{Digest, Keccak256};

fn main() {
    let function_name = "getAnotherValue()"; // should output 0x25d805cf
    let hashed_data = Keccak256::digest(function_name);

    println!("0x{:x}{:x}{:x}{:x}", hashed_data[0], hashed_data[1], hashed_data[2], hashed_data[3]);
}
