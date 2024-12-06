#![no_std]

use t_rust::{commit, read};

pub fn main() {
    let n = read::<u32>();

    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let mut c = a + b;
        c %= 7919; // Modulus to prevent overflow.
        a = b;
        b = c;
    }

    commit::<u32>(&b);
}
