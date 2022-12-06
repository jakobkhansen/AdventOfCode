use std::{
    error::Error,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    first()
}

fn first() -> Result<(), Box<dyn Error>> {
    let io: io::Stdin = io::stdin();
    let lines: Vec<String> = io.lock().lines().collect::<Result<Vec<String>, _>>()?;
    let mut sum = 0;
    for line in lines {
        let mut it = line.split(",");
        let first_pair = it.next().unwrap();
        let second_pair = it.next().unwrap();

        let first_val: Vec<i32> = first_pair.split("-").map(|x| x.parse().unwrap()).collect();
        let second_val: Vec<i32> = second_pair.split("-").map(|x| x.parse().unwrap()).collect();

        sum += if overlap(first_val, second_val) { 1 } else { 0 };
    }

    println!("{}", sum);

    Ok(())
}

fn contained(first: Vec<i32>, sec: Vec<i32>) -> bool {
    if first[0] <= sec[0] && first[1] >= sec[1] {
        return true;
    }
    if sec[0] <= first[0] && sec[1] >= first[1] {
        return true;
    }

    false
}

fn overlap(first: Vec<i32>, sec: Vec<i32>) -> bool {
    (first[0] <= sec[1]) && (first[1] >= sec[0])
}
