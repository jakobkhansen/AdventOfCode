use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    // first()
    second()
}

fn first() -> Result<(), Box<dyn Error>> {
    let io: io::Stdin = io::stdin();
    let lines: Vec<String> = io.lock().lines().collect::<Result<Vec<String>, _>>()?;

    let mut score = 0;
    for line in lines {
        let mut spl = line.split(" ");
        let opponent_char: char = spl.next().unwrap().parse().unwrap();
        let me_char: char = spl.next().unwrap().parse().unwrap();

        let opponent: u32 = opponent_char as u32 - 'A' as u32;
        let me: u32 = me_char as u32 - 'X' as u32;
        score += determine_shape_score(me) + determine_result(me, opponent);
    }

    println!("{}", score);
    Ok(())
}

fn determine_result(me: u32, opponent: u32) -> u32 {
    let wins_against: HashMap<u32, u32> = HashMap::from([(0, 2), (1, 0), (2, 1)]);
    if me == opponent {
        return 3;
    }

    if wins_against.get(&me).unwrap().to_owned() == opponent {
        return 6;
    }

    return 0;
}

fn determine_shape_score(me: u32) -> u32 {
    let scores: HashMap<u32, u32> = HashMap::from([(0, 1), (1, 2), (2, 3)]);

    return scores.get(&me).unwrap().to_owned();
}

fn second() -> Result<(), Box<dyn Error>> {
    let io: io::Stdin = io::stdin();
    let lines: Vec<String> = io.lock().lines().collect::<Result<Vec<String>, _>>()?;

    let mut score = 0;
    for line in lines {
        let mut spl = line.split(" ");
        let opponent_char: char = spl.next().unwrap().parse().unwrap();
        let strategy_char: char = spl.next().unwrap().parse().unwrap();

        let opponent: u32 = opponent_char as u32 - 'A' as u32;
        let strategy: u32 = strategy_char as u32 - 'X' as u32;
        let me = get_correct_move(strategy, opponent);
        score += determine_shape_score(me) + determine_result(me, opponent);
    }

    println!("{}", score);
    Ok(())
}

fn get_correct_move(result: u32, opponent: u32) -> u32 {
    let winning: HashMap<u32, u32> = HashMap::from([(2, 0), (0, 1), (1, 2)]);
    let losing: HashMap<u32, u32> = HashMap::from([(0, 2), (1, 0), (2, 1)]);
    let draw: HashMap<u32, u32> = HashMap::from([(0, 0), (1, 1), (2, 2)]);

    let strategies: HashMap<u32, HashMap<u32, u32>> =
        HashMap::from([(0, losing), (1, draw), (2, winning)]);

    return strategies
        .get(&result)
        .unwrap()
        .get(&opponent)
        .unwrap()
        .to_owned();
}
