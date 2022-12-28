use std::collections::HashSet;
use aoc_2022::{read_vector_of_string};

pub fn read_input() -> String {
    String::from(&read_vector_of_string("input/day06.txt")[0])
}

pub fn find_first_unique_substring(s : &str, len: usize) -> &str {
    for i in 0..s.len() - len {
        let substring = &s[i..i + len];
        if unique(substring) {
            return substring;
        }
    }
    panic!("found no unique substring with {} letters", len)
}

fn unique(s: &str) -> bool {
    let set: HashSet<char> = HashSet::from_iter(s.chars());
    set.len() == s.len()
}

pub fn find_end_of_marker(input: &str, len: usize) -> usize {
    let marker = find_first_unique_substring(input, len);
    input.find(marker).expect("should find unique substring") + len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_read_input() {
        let result = read_input();
        assert_eq!(result.starts_with("gzbzw"), true);
    }

    #[test]
    fn test_can_find_end_of_marker() {
        assert_eq!(find_end_of_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find_end_of_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(find_end_of_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(find_end_of_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);

        assert_eq!(find_end_of_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find_end_of_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(find_end_of_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(find_end_of_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }

    #[test]
    fn run_part_1_with_puzzle_input() {
        let input = read_input();
        assert_eq!(find_end_of_marker(&input, 4), 1262);
    }

    #[test]
    fn run_part_2_with_puzzle_input() {
        let input = read_input();
        assert_eq!(find_end_of_marker(&input, 14), 3444);
    }
}

fn main() {}
