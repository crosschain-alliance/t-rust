#![cfg_attr(feature = "guest", no_std)]
use t_rust::{commit, read};
extern crate userscrate;

#[jolt::provable]
fn entrypoint(n: u32) -> u32 {
    commit::<u32>(&n);
    userscrate::main();
    let sum = read::<u32>();
    sum
}