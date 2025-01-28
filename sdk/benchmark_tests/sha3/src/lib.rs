#![no_std]

extern crate alloc;
use t_rust::{commit, read_vec};
use alloc::vec::Vec;

use sha3::{Digest, Keccak256};

pub fn main() {
    let input: Vec<u8> = read_vec();

    let mut hasher = Keccak256::new();
    hasher.update(input);
    let result = hasher.finalize();

    commit::<[u8; 32]>(&result.into());
}

