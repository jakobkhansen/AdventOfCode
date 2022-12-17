use std::{
    collections::{HashMap, HashSet},
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

fn first(lines: &Vec<String>) {
    let dir = HashMap::from([('R', (0, 1)), ('U', (-1, 0)), ('D', (1, 0)), ('L', (0, -1))]);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut rope = [(0, 0); 10];
    for line in lines {
        let mut words = line.split(" ");

        let dir_char = words.next().unwrap().chars().nth(0).unwrap();
        let num_moves = words.next().unwrap().parse().unwrap();

        let curr_move = dir[&dir_char];

        for _ in 0..num_moves {
            drag(&mut rope, curr_move);
            visited.insert(rope[rope.len() - 1]);
        }
    }
    println!("{}", visited.len());
}
fn second(lines: &Vec<String>) {}

fn drag(rope: &mut [(i32, i32); 10], curr_move: (i32, i32)) {
    rope[0].0 += curr_move.0;
    rope[0].1 += curr_move.1;
    for i in 0..rope.len() - 1 {
        let (diff_x, diff_y) = ((rope[i].0 - rope[i + 1].0), (rope[i].1 - rope[i + 1].1));

        if diff_x.abs() > 1 || diff_y.abs() > 1 {
            rope[i + 1].0 += sign(diff_x);
            rope[i + 1].1 += sign(diff_y);
        }
    }
}

fn sign(d: i32) -> i32 {
    if d == 0 {
        0
    } else {
        d / d.abs()
    }
}
