// #![no_std]
extern crate alloc;
use alloy_rlp::{Decodable};
use reth_chainspec::MAINNET;
use reth_consensus_common::validation::validate_block_pre_execution;
use reth_primitives::{Block};

use t_rust::{commit, read_vec};

use alloc::vec::Vec;

pub fn main() {

    let buf_slice: Vec<u8> = read_vec();

    let loaded_reth_block = Block::decode(&mut buf_slice.as_slice()).unwrap();
    let sealed_block = loaded_reth_block.seal_slow();
    match validate_block_pre_execution(&sealed_block, &MAINNET.clone()) {
        Ok(()) => commit::<u32>(&0),
        Err(_) => commit::<u32>(&1),
    }
}
