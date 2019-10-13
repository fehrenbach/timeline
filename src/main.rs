use std::env;
use std::io::prelude::*;
use std::io;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[..] {
        [_, first] => match first as &str {
            "--this" => this(),
            "--next" => next(),
            _ => println!("expecting exactly one of --this or --next"),
        }
        _ => println!("expecting exactly one of --this or --next"),
    }
}

fn this() {
    let mut now = Instant::now();
    for line in io::stdin().lock().lines() {
        println!("{} {}", now.elapsed().as_nanos(), line.unwrap());
        now = Instant::now();
    }
}

fn next() {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(0) => return,
        _ => {}
    }
    let mut now = Instant::now();
    loop {
        let mut new_line = String::new();
        let eof = match io::stdin().read_line(&mut new_line) {
            Ok(0) => true,
            _ => false,
        };
        print!("{} {}", now.elapsed().as_nanos(), line);
        now = Instant::now();
        line = new_line;
        if eof { break; }
    }
}
