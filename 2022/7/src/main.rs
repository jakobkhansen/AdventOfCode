use std::{
    collections::HashMap,
    error::Error,
    fmt::Debug,
    io::{self, BufRead},
};

struct Filesystem {
    root: Box<Node>,
}
struct Node {
    file_size_sum: i32,
    children: HashMap<String, Box<Node>>,
}
impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("file_size_sum", &self.file_size_sum)
            .field("children", &self.children)
            .finish()
    }
}

impl Debug for Filesystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Filesystem")
            .field("root", &self.root)
            .finish()
    }
}

impl Filesystem {
    fn insert_file(&mut self, path: &mut Vec<String>, file_size: i32) {
        self.root.insert_file(path, file_size)
    }
    fn insert_dir(&mut self, path: &mut Vec<String>, name: String) {
        self.root.insert_dir(path, name)
    }

    fn get_folder_sizes(&self) -> i32 {
        self.root.get_folder_size()
    }

    fn find_dirs_above_threshold(&self, threshold: i32) {
        self.root.find_dirs_above_threshold(threshold);
    }
}

impl Node {
    fn insert_file(&mut self, path: &mut Vec<String>, file_size: i32) {
        if path.len() <= 0 {
            self.file_size_sum += file_size;
            return;
        }

        let mut new_path = path.to_vec();
        let next_folder = new_path.remove(0);
        let child = self.children.get_mut(&next_folder).unwrap();
        child.insert_file(&mut new_path, file_size);
    }
    fn insert_dir(&mut self, path: &mut Vec<String>, name: String) {
        if path.len() <= 1 {
            self.children.insert(
                name,
                Box::new(Node {
                    file_size_sum: 0,
                    children: HashMap::new(),
                }),
            );
            return;
        }

        let mut new_path = path.to_vec();
        let next_folder = new_path.remove(0);
        let child = self.children.get_mut(&next_folder).unwrap();
        child.insert_dir(&mut new_path, name);
    }
    fn get_folder_size(&self) -> i32 {
        let subdir_sum: i32 = self
            .children
            .values()
            .map(|child| child.get_folder_size())
            .sum();
        let sum = subdir_sum + self.file_size_sum;

        if sum <= 100000 {
            println!("{sum}")
        }

        sum
    }
    fn find_dirs_above_threshold(&self, threshold: i32) -> i32 {
        let subdir_sum: i32 = self
            .children
            .values()
            .map(|child| child.find_dirs_above_threshold(threshold))
            .sum();
        let sum = subdir_sum + self.file_size_sum;

        if sum >= threshold {
            println!("{sum}")
        }

        sum
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let io: io::Stdin = io::stdin();
    let lines = io.lock().lines().collect::<Result<Vec<String>, _>>()?;

    let mut fs = Filesystem {
        root: Box::new(Node {
            file_size_sum: 0,
            children: HashMap::new(),
        }),
    };

    let mut path: Vec<String> = vec![];
    for line in &lines[1..] {
        match line.chars().nth(0).unwrap() {
            '$' => match line.split(" ").nth(1).unwrap() {
                "ls" => {}
                "cd" => match line.split(" ").nth(2).unwrap() {
                    ".." => {
                        path.pop();
                    }
                    i => {
                        path.push(i.to_string());
                        fs.insert_dir(&mut path, i.to_string());
                    }
                },
                _ => {
                    panic!("Invalid command")
                }
            },
            'd' => {}
            _ => {
                let size = line.split(" ").nth(0).unwrap().parse::<i32>()?;
                fs.insert_file(&mut path, size);
            }
        }
    }

    let total_space = 70000000;
    let used_space = fs.get_folder_sizes();
    let available_space = total_space - used_space;
    let goal_space = 30000000;
    let min_delete_size = goal_space - available_space;
    println!("{min_delete_size}");
    println!("");

    fs.find_dirs_above_threshold(min_delete_size);

    Ok(())
}
