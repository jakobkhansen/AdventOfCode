use std::{error::Error, io};

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    println!("{}", first(&input));
    println!("{}", second(&input));
}

fn first(input: &String) -> i32 {
    0
}
fn second(input: &String) -> i32 {
    0
}
