use serde::{de::DeserializeOwned, Serialize};
use sp1_zkvm::io;

pub fn commit<T: Serialize>(value: &T) {
    io::commit(&value);
}

pub fn commit_slice(value: &[u8]) {
    io::commit_slice(&value);
}

pub fn read<T: DeserializeOwned>() -> T {
    io::read::<T>()
}

pub fn read_vec() -> Vec<u8> {
    io::read_vec()
}
