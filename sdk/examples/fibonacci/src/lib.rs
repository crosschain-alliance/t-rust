#![no_std]

pub fn main(){
    t_rust::greetings();

    let n = 42;

    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let mut c = a + b;
        c %= 7919; // Modulus to prevent overflow.
        a = b;
        b = c;
    }

    t_rust::commit_u64(&b);
}
