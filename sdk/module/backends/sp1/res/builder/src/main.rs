#![no_main]

extern crate userscrate;

sp1_zkvm::entrypoint!(userscrate::main);
