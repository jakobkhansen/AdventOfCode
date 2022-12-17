use std::{
    error::Error,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let io: io::Stdin = io::stdin();
    let lines = io.lock().lines().collect::<Result<Vec<String>, _>>()?;

    first(&lines);
    second(&lines);
    Ok(())
}

fn first(lines: &Vec<String>) {}
fn second(lines: &Vec<String>) {}
