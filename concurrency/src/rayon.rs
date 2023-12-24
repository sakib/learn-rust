use rayon::prelude::*;
use num::BigUint;
use num::One;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    factorial(50000);
    println!("{:.2?}", now.elapsed());
    let now = Instant::now();
    multi_threaded_factorial(50000);
    println!("{:.2?}", now.elapsed());
}

fn factorial(num: u32) -> BigUint {
    if [0,1].contains(&num) {
        BigUint::one()
    } else {
        (1..=num).map(BigUint::from).reduce(|acc, x| acc * x).unwrap()
    }
}

fn multi_threaded_factorial(num: u32) -> BigUint {
    if [0,1].contains(&num) {
        BigUint::one()
    } else {
        // (1..=num).map(BigUint::from).reduce(|acc, x| acc * x).unwrap()
        (1..=num).into_par_iter().map(BigUint::from).reduce(|| BigUint::one(), |acc, x| acc * x)
    }
}