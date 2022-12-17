use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    // first()?;
    second()?;

    Ok(())
}

fn first() -> Result<(), Box<dyn Error>> {
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
    for (cycles, diff) in instructions {
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

fn second() -> Result<(), Box<dyn Error>> {
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
    let mut next_checkpoint = 40;
    let mut screen: Vec<String> = vec![];
    for (cycles, diff) in instructions {
        for _ in 0..cycles.to_owned() {
            let pixel_x = (curr_cycle + 40) - next_checkpoint;
            println!("cycle {curr_cycle} x {} pixel_x {}", x, pixel_x);
            if (x - 1) <= pixel_x && (x + 1) >= pixel_x {
                screen.push(String::from("#"));
            } else {
                screen.push(String::from("."));
            }
            curr_cycle += 1;
            if curr_cycle >= next_checkpoint {
                next_checkpoint += 40;
                println!();
                screen.push(String::from("\n"));
            }
        }

        x += diff;
    }
    println!("{}", screen.join(""));
    Ok(())
}
