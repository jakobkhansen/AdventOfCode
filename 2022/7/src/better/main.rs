use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    io::{self, BufRead},
    rc::Rc,
};

#[derive(Debug)]
struct Filesystem {
    root: Box<Node>,
}

#[derive(Debug)]
struct Node {
    file_size_sum: i32,
    children: HashMap<String, Node>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let io: io::Stdin = io::stdin();
    let lines = io.lock().lines().collect::<Result<Vec<String>, _>>()?;

    let mut root = Node {
        file_size_sum: 0,
        children: HashMap::new(),
    };
    let mut current: &mut Node = &mut root;
    for line in &lines[1..] {
        match line.chars().nth(0).unwrap() {
            '$' => match line.split(" ").nth(1).unwrap() {
                "ls" => {}
                "cd" => match line.split(" ").nth(2).unwrap() {
                    ".." => {}
                    i => {
                        current = if current.children.contains_key(&i.to_string()) {
                            current.children.get_mut(&i.to_string()).unwrap()
                        } else {
                            let new_node = Node {
                                file_size_sum: 0,
                                children: HashMap::new(),
                            };
                            current.children.insert(i.to_string(), new_node);
                            current.children.get_mut(&i.to_string()).unwrap()
                        };
                    }
                },
                _ => {
                    panic!("Invalid command")
                }
            },
            'd' => {}
            _ => {
                let size = line.split(" ").nth(0).unwrap().parse::<i32>()?;
            }
        }
    }

    println!("{:?}", root);
    Ok(())
}
