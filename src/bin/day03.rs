use std::collections::HashSet;
use aoc_2022::{read_vector_of_string};

pub fn read_input() -> Vec<String> {
    read_vector_of_string("input/day03.txt")
}

/// Splits a string in two equally long halves.
pub fn split_in_half(s: &str) -> Vec<&str> {
    if s.len() % 2 != 0 {
        panic!("Odd string length: {}", s.len());
    }
    let half = s.len() / 2;
    vec![&s[..half], &s[half..]]
}

/// Finds the first char in 's' that is also in 'chars_to_find'.
pub fn find_first(s: &str, chars_to_find: &str) -> char {
    let found: String = chars_to_find.chars().filter(|c| s.contains(*c)).collect();
    found.chars().next().expect("Should find a common char")
}

/// Returns the priority of the given char.
pub fn get_priority(c: char) -> i32 {
    if c.is_lowercase() {
        (c as i32) - 96
    } else {
        (c as i32) - 38
    }
}

/// Finds the single char that is common to all strings in 'v'.
pub fn find_common_char(v: &Vec<&str>) -> char {
    let mut common_chars = to_set(v[0]);

    for set in v.iter().map(|s| to_set(s)) {
        common_chars = HashSet::from_iter(common_chars.intersection(&set).map(|c| *c));
    }
    assert_eq!(common_chars.len(), 1);

    *common_chars.iter().next().expect("Should find a common char")
}

/// Converts the chars of a string into a set of unique chars.
fn to_set(s: &str) -> HashSet<char> {
    HashSet::from_iter(s.chars())
}

#[cfg(test)]
mod tests {
    use aoc_2022::{split_into_groups, to_vector_of_str};
    use super::*;

    #[test]
    fn test_can_read_input() {
        let result = read_input();
        assert_eq!(result[0], "jLnFTjhwFTLFDGDDvLgvDssBJBbVRNZJPPJBGzBNRVJNRB");
    }

    #[test]
    fn test_can_split_string() {
        let result = split_in_half("jLnFTjhwFTLFDGDDvLgvDssBJBbVRNZJPPJBGzBNRVJNRB");
        assert_eq!(result, vec!["jLnFTjhwFTLFDGDDvLgvDss", "BJBbVRNZJPPJBGzBNRVJNRB"]);
    }

    #[test]
    fn test_can_find_first() {
        assert_eq!(find_first("jLnFTjhwFTLFDGDDvLgvDss", "BJBbVRNZJPPJBGzBNRVJNRB"), 'G');
        assert_eq!(find_first("vJrwpWtwJgWr", "hcsFMMfFFhFp"), 'p');
    }

    #[test]
    fn test_can_get_priority() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);
    }

    #[test]
    fn test_can_find_common_char() {
        let result = find_common_char(&vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ]);
        assert_eq!(result, 'r');

        let result = find_common_char(&vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]);
        assert_eq!(result, 'Z');
    }

    #[test]
    fn run_part_1_with_puzzle_input() {
        let sum: i32 = read_input().iter()
            .map(|s| split_in_half(s))
            .map(|v| find_first(v[0], v[1]))
            .map(|c| get_priority(c))
            .sum();
        assert_eq!(sum, 7903);
    }

    #[test]
    fn run_part_2_with_puzzle_input() {
        let input = read_input();
        let sum: i32 = split_into_groups(to_vector_of_str(&input), 3).iter()
            .map(|group| find_common_char(group))
            .map(|c| get_priority(c))
            .sum();
        assert_eq!(sum, 2548);
    }
}

fn main() {}
