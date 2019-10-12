use std::io;
use std::io::prelude::*;
use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    for line in io::stdin().lock().lines() {
        println!("{} {}", now.elapsed().as_nanos(), line.unwrap());
        now = Instant::now();
    }
}
