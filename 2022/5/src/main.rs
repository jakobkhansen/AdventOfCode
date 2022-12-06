use std::{
    error::Error,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let io: io::Stdin = io::stdin();
    let lines: Vec<String> = io.lock().lines().collect::<Result<Vec<String>, _>>()?;

    let mut line_iter = lines.iter();

    let mut matrix: Vec<Vec<char>> = vec![];
    while let Some(line) = line_iter.next() {
        if line == "" {
            break;
        }
        let mut matrix_line: Vec<char> = vec![];
        for i in (0..line.len()).step_by(4) {
            matrix_line.push(line.chars().nth(i + 1).unwrap());
        }
        matrix.push(matrix_line);
    }
    matrix.pop();
    println!("{:?}", matrix);

    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..matrix.len() + 1 {
        stacks.push(vec![]);
    }

    for line in matrix.iter().rev() {
        for (i, elem) in line.iter().enumerate() {
            if elem.to_owned() != ' ' {
                stacks[i].push(elem.to_owned());
            }
        }
    }
    println!("{:?}", stacks);

    for line in line_iter {
        let count: usize = line.split(" ").nth(1).unwrap().parse().unwrap();
        let from: usize = line.split(" ").nth(3).unwrap().parse::<usize>().unwrap() - 1;
        let to: usize = line.split(" ").nth(5).unwrap().parse::<usize>().unwrap() - 1;

        let mut push_elements: Vec<char> = vec![];
        for _ in 0..count {
            let from_stack = stacks.get_mut(from).unwrap();
            push_elements.push(from_stack.pop().unwrap());
        }
        let to_stack = stacks.get_mut(to).unwrap();

        for elem in push_elements.iter().rev() {
            to_stack.push(elem.to_owned());
        }
    }
    println!("{:?}", stacks);
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();

    Ok(())
}
