#![cfg_attr(feature = "guest", no_std)]
use t_rust::{commit, read};
extern crate userscrate;

extern crate alloc;
use alloc::vec::Vec;

#[jolt::provable]
fn entrypoint(n: u32) -> u32 {
    commit::<u32>(&n);
    userscrate::main();
    let sum = read::<u32>();
    sum
}

#[jolt::provable]
fn entrypoint_vec(n: &[u8]) -> [u8; 32] {
    commit::<Vec<u8>>(&n.to_vec());
    userscrate::main();
    let sum = read::<[u8; 32]>();
    sum
}
