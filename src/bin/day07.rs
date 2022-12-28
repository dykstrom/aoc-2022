use std::collections::HashMap;
use aoc_2022::{read_vector_of_string, to_i64};

pub fn read_input() -> Vec<String> {
    read_vector_of_string("input/day07.txt")
}

pub fn get_sizes(mut vec: &[&str]) -> HashMap<String, i64> {
    let mut sizes: HashMap<String, i64> = HashMap::new();
    let mut stack: Vec<(&str, i64)> = Vec::new();

    let mut current_size = 0;

    while !vec.is_empty() {
        let parts: Vec<&str> = vec[0].split_whitespace().collect();
        if parts[0] == "$" && parts[1] == "ls" {
            // Ignore
        } else if parts[0] == "$" && parts[1] == "cd" {
            // Change directory command
            if parts[2] == ".." {
                // When returning from directory, retrieve its (full) name
                // and the size before entering
                let path = get_path(&stack);
                let dir_size = stack.pop().expect("stack should contain directory").1;
                sizes.insert(path, current_size);
                current_size += dir_size;
            } else {
                // Save the name of the new directory, and the size before entering it
                stack.push((parts[2], current_size));
                current_size = 0;
            }
        } else if parts[0] == "dir" {
            // Ignore
        } else {
            // Regular file
            current_size += to_i64(parts[0]);
        }

        vec = &vec[1..];
    }

    // Save remaining pushed directories and their size
    while !stack.is_empty() {
        let path = get_path(&stack);
        let dir_size = stack.pop().expect("stack should contain directory").1;
        sizes.insert(path, current_size);
        current_size += dir_size;
    }

    sizes
}

/// Constructs a "/" separated path from the directory names in 'stack'.
fn get_path(stack: &Vec<(&str, i64)>) -> String {
    stack.iter()
        .map(|t| String::from(t.0))
        .fold(String::new(), |a, b| (a + "/" + &b).replace("//", "/"))
}

#[cfg(test)]
mod tests {
    use aoc_2022::to_vector_of_str;
    use super::*;

    #[test]
    fn test_can_read_input() {
        let input = read_input();
        assert_eq!(input[0], "$ cd /");
        assert_eq!(input[1], "$ ls");
        assert_eq!(input[2], "dir dpbwg");
    }

    #[test]
    fn test_get_size_of_empty() {
        let vec: Vec<&str> = vec![];
        let sizes = get_sizes(&vec);
        assert_eq!(sizes.len(), 0);
    }

    #[test]
    fn test_get_size_of_single_cd() {
        let vec: Vec<&str> = vec!["$ cd /"];
        let sizes = get_sizes(&vec);
        assert_eq!(sizes.len(), 1);
        assert_eq!(sizes.get("/"), Some(&0));
    }

    #[test]
    fn test_get_size_of_single_ls() {
        let vec: Vec<&str> = vec!["$ cd foo", "$ ls"];
        let sizes = get_sizes(&vec);
        assert_eq!(sizes.len(), 1);
        assert_eq!(sizes.get("/foo"), Some(&0));
    }

    #[test]
    fn test_get_size_of_ls_single_file() {
        let vec: Vec<&str> = vec!["$ cd foo", "$ ls", "100 c.dat"];
        let sizes = get_sizes(&vec);
        assert_eq!(sizes.len(), 1);
        assert_eq!(sizes.get("/foo"), Some(&100));
    }

    #[test]
    fn test_get_size_of_ls_some_files() {
        let vec: Vec<&str> = vec!["$ cd /", "$ ls", "100 c.dat", "66 a.txt", "1 a.out"];
        let sizes = get_sizes(&vec);
        assert_eq!(sizes.len(), 1);
        assert_eq!(sizes.get("/"), Some(&167));
    }

    #[test]
    fn test_get_size_of_cd_in_and_out() {
        let vec: Vec<&str> = vec!["$ cd /", "$ cd foo", "$ cd .."];
        let sizes = get_sizes(&vec);
        assert_eq!(sizes.len(), 2);
        assert_eq!(sizes.get("/"), Some(&0));
        assert_eq!(sizes.get("/foo"), Some(&0));
    }

    #[test]
    fn test_get_size_of_cd_in_and_out_with_file() {
        let vec: Vec<&str> = vec!["$ cd /", "$ cd foo", "$ ls", "8 hej.txt", "$ cd .."];
        let sizes = get_sizes(&vec);
        assert_eq!(sizes.len(), 2);
        assert_eq!(sizes.get("/"), Some(&8));
        assert_eq!(sizes.get("/foo"), Some(&8));
    }

    #[test]
    fn test_get_size_of_cd_in_and_out_with_some_files() {
        let vec: Vec<&str> = vec![
            "$ cd /",
            "$ ls",
            "dir foo",
            "5 a.out",
            "$ cd foo",
            "$ ls",
            "8 hej.txt",
            "3 hoj.txt",
            "$ cd ..",
        ];
        let sizes = get_sizes(&vec);
        assert_eq!(sizes.len(), 2);
        assert_eq!(sizes.get("/"), Some(&16));
        assert_eq!(sizes.get("/foo"), Some(&11));
    }

    #[test]
    fn test_get_size_of_cd_in_and_out_with_some_directories() {
        let vec: Vec<&str> = vec![
            "$ cd /",
            "$ ls",
            "dir foo",
            "dir bar",
            "5 a.out",
            "$ cd foo",
            "$ ls",
            "8 hej.txt",
            "3 hoj.txt",
            "$ cd ..",
            "$ cd bar",
            "$ ls",
            "dir tee",
            "90 guh.txt",
            "$ cd tee",
            "$ ls",
            "1 small.txt",
        ];
        let sizes = get_sizes(&vec);
        assert_eq!(sizes.len(), 4);
        assert_eq!(sizes.get("/"), Some(&107));
        assert_eq!(sizes.get("/foo"), Some(&11));
        assert_eq!(sizes.get("/bar"), Some(&91));
        assert_eq!(sizes.get("/bar/tee"), Some(&1));
    }

    #[test]
    fn test_get_size_with_example() {
        let vec: Vec<&str> = vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];
        let sizes = get_sizes(&vec);
        assert_eq!(sizes.len(), 4);
        assert_eq!(sizes.get("/"), Some(&48381165));
        assert_eq!(sizes.get("/a"), Some(&94853));
        assert_eq!(sizes.get("/a/e"), Some(&584));
        assert_eq!(sizes.get("/d"), Some(&24933642));
    }

    #[test]
    fn run_part_1_with_puzzle_input() {
        let input = read_input();
        let vec = to_vector_of_str(&input);
        let sizes = get_sizes(&vec);
        let sum_of_sizes: i64 = sizes.iter()
            .filter(|i| *i.1 <= 100_000)
            .map(|i| *i.1)
            .sum();
        assert_eq!(sum_of_sizes, 1350966);
    }

    #[test]
    fn run_part_2_with_puzzle_input() {
        let total_space = 70_000_000;
        let required_space = 30_000_000;

        let input = read_input();
        let vec = to_vector_of_str(&input);
        let sizes = get_sizes(&vec);
        let used_space = sizes.get("/").expect("root has a size");
        let free_space = total_space - used_space;
        let missing_space = required_space - free_space;
        assert!(missing_space > 0);

        // Sort the list of directory sizes in increasing order
        let mut list: Vec<i64> = sizes.iter()
            .map(|i| *i.1)
            .collect();
        list.sort();

        // Find the smallest directory that is still bigger than the missing space
        let size_of_fittest_dir = list.iter()
            .find(|size| **size > missing_space)
            .expect("one directory is big enough");
        assert_eq!(*size_of_fittest_dir, 6296435);
    }
}

fn main() {}
