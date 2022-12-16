use std::{
    error::Error,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let io: io::Stdin = io::stdin();
    let lines = io.lock().lines().collect::<Result<Vec<String>, _>>()?;

    let mut matrix: Vec<Vec<u32>> = vec![];

    for line in lines {
        let mut row: Vec<u32> = vec![];
        for char in line.chars() {
            row.push(char.to_digit(10).unwrap())
        }
        matrix.push(row);
    }

    let rows = matrix.len();
    let col = matrix.get(0).unwrap().len();
    let mut visible = rows * 2 + (col - 2) * 2;
    let mut best_scenic_score = 0;

    for i in 1..rows - 1 {
        for j in 1..col - 1 {
            let tree = matrix.get(i).unwrap().get(j).unwrap();
            let mut north = false;
            let mut south = false;
            let mut east = false;
            let mut west = false;

            let mut scenic = 1;

            let mut west_scenic = 0;
            for k in (0..j).rev() {
                west_scenic += 1;
                if matrix.get(i).unwrap().get(k).unwrap() >= tree {
                    west = true;
                    break;
                }
            }
            scenic *= west_scenic;

            let mut east_scenic = 0;
            for k in j + 1..col {
                east_scenic += 1;
                if matrix.get(i).unwrap().get(k).unwrap() >= tree {
                    east = true;
                    break;
                }
            }
            scenic *= east_scenic;

            let mut north_scenic = 0;
            for k in (0..i).rev() {
                north_scenic += 1;
                if matrix.get(k).unwrap().get(j).unwrap() >= tree {
                    north = true;
                    break;
                }
            }
            scenic *= north_scenic;

            let mut south_scenic = 0;
            for k in i + 1..rows {
                south_scenic += 1;
                if matrix.get(k).unwrap().get(j).unwrap() >= tree {
                    south = true;
                    break;
                }
            }
            scenic *= south_scenic;

            if !west || !north || !east || !south {
                visible += 1
            }
            println!("{i} {j} {scenic}");

            best_scenic_score = std::cmp::max(scenic, best_scenic_score)
        }
    }
    println!("Visible (part 1) {visible}");
    println!("Best scenic score (part 2) {best_scenic_score}");

    Ok(())
}
