use std::cmp;
use std::error::Error;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    // first();
    second()
}

fn first() -> Result<(), Box<dyn Error>> {
    let io: io::Stdin = io::stdin();
    let lines: Result<Vec<String>, _> = io.lock().lines().collect();

    let mut current = 0;
    let mut max = 0;

    for line in lines? {
        if line == "" {
            max = cmp::max(current, max);
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }
    println!("{}", max);
    Ok(())
}

fn second() -> Result<(), Box<dyn Error>> {
    let io: io::Stdin = io::stdin();
    let lines: Result<Vec<String>, _> = io.lock().lines().collect();

    let mut current = 0;
    let mut vec = vec![];

    for line in lines? {
        if line == "" {
            vec.push(current);
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();
        }
    }
    vec.sort();
    vec.reverse();
    println!("{:?}", vec[0..3].to_vec());
    println!("{}", vec[0..3].iter().sum::<i32>());
    Ok(())
}
