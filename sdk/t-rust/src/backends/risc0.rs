use serde::{de::DeserializeOwned, Serialize};
use risc0_zkvm::guest::env;

pub fn commit<T: Serialize>(value: &T) {
    env::commit(&value);
}

pub fn write<T: Serialize>(data: &T) {
    env::write(&data);
}

pub fn read<T: DeserializeOwned>() -> T {
    env::read::<T>()
}