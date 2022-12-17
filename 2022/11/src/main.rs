use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let input = io::read_to_string(io::stdin())?;
    let instructions: Vec<(i32, i32)> = input
        .lines()
        .map(|line| line.split_once(" ").or(Some(("noop", ""))).unwrap())
        .map(|(instruction, rest)| match instruction {
            "addx" => (2, rest.parse::<i32>().unwrap()),
            "noop" => (1, 0),
            _ => unreachable!(),
        })
        .collect();

    let mut x = 1;
    let mut curr_cycle = 0;
    let mut res = 0;
    let mut next_checkpoint = 20;

    for (i, (cycles, diff)) in instructions.iter().enumerate() {
        curr_cycle += cycles;
        if curr_cycle >= next_checkpoint {
            res += x * next_checkpoint;
            next_checkpoint += 40;
        }

        x += diff;
    }
    println!("{}", res);

    Ok(())
}
