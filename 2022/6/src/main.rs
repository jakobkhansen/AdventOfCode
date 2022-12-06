use std::{
    collections::HashMap,
    error::Error,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let io: io::Stdin = io::stdin();
    let lines = io.lock().lines().collect::<Result<Vec<String>, _>>()?;

    let mut map: HashMap<char, i32> = HashMap::new();
    let input = lines.first().unwrap();

    let window_size = 14;

    for i in 0..window_size - 1 {
        let char = input.chars().nth(i).unwrap();
        let existing_val = map.entry(char).or_default().to_owned();
        map.insert(char, existing_val + 1);
        println!("{i}")
    }
    println!("");
    for i in (window_size - 1)..input.len() {
        println!("{i}");
        let char = input.chars().nth(i).unwrap();
        let existing_val = map.entry(char).or_default().to_owned();
        map.insert(char, existing_val + 1);

        let mut sum = 0;
        for key in map.keys() {
            println!("{key} {}", map.get(key).unwrap());
            if map.get(key).unwrap().to_owned() == 1 {
                sum += 1;
            }
        }

        println!("i {i} {sum}");
        if sum == window_size {
            println!("{}", i + 1);
            return Ok(());
        }

        let remove_char = input.chars().nth(i - (window_size - 1)).unwrap();
        let remove_val = map.entry(remove_char).or_default().to_owned();
        println!(
            "deleting index {} with value {}",
            i - (window_size - 1),
            remove_val
        );
        map.insert(remove_char, remove_val - 1);
    }

    Ok(())
}
