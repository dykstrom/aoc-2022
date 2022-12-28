use lazy_static::lazy_static;
use regex::{Regex};
use aoc_2022::{read_vector_of_string, to_i32};

#[derive(Debug, Eq, PartialEq)]
pub struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    pub fn from(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^move ([0-9]+) from ([1-9]) to ([1-9])$").unwrap();
        }
        match RE.captures(s) {
            Some(capture) => {
                Move {
                    count: to_i32(&capture[1]) as usize,
                    from: to_i32(&capture[2]) as usize,
                    to: to_i32(&capture[3]) as usize,
                }
            }
            None => panic!("failed to parse move: {}", s)
        }
    }

    /// Executes a move by actually moving crates from one stack to another.
    /// This method moves one crate at a time.
    pub fn execute_1(&self, stacks: &mut Vec<Vec<char>>) {
        for _n in 0..self.count {
            match stacks[self.from].pop() {
                Some(c) => stacks[self.to].push(c),
                None => panic!("stack {} is exhausted", self.from)
            }
        }
    }

    /// Executes a move by actually moving crates from one stack to another.
    /// This method moves all crates at once.
    pub fn execute_2(&self, stacks: &mut Vec<Vec<char>>) {
        let len = stacks[self.from].len();
        let mut sub_stack: Vec<char> = stacks[self.from].drain(len - self.count..len).collect();
        stacks[self.to].append(&mut sub_stack);
    }
}

pub fn get_top_of_stacks(stacks: &Vec<Vec<char>>) -> String {
    let mut result = String::new();
    stacks.iter()
        .filter(|s| !s.is_empty())
        .map(|s| s[s.len() - 1])
        .for_each(|c| result.push(c));
    assert_eq!(result.len(), 9);
    result
}

pub fn create_stacks() -> Vec<Vec<char>> {
    vec![
        vec![], // Not used
        vec!['H', 'C', 'R'],
        vec!['B', 'J', 'H', 'L', 'S', 'F'],
        vec!['R', 'M', 'D', 'H', 'J', 'T', 'Q'],
        vec!['S', 'G', 'R', 'H', 'Z', 'B', 'J'],
        vec!['R', 'P', 'F', 'Z', 'T', 'D', 'C', 'B'],
        vec!['T', 'H', 'C', 'G'],
        vec!['S', 'N', 'V', 'Z', 'B', 'P', 'W', 'L'],
        vec!['R', 'J', 'Q', 'G', 'C'],
        vec!['L', 'D', 'T', 'R', 'H', 'P', 'F', 'S'],
    ]
}

pub fn read_input() -> Vec<String> {
    read_vector_of_string("input/day05.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_read_input() {
        let result = read_input();
        assert_eq!(result[0], "move 8 from 7 to 1");
    }

    #[test]
    fn test_can_create_stacks() {
        let stacks = create_stacks();
        assert_eq!(stacks[1], vec!['H', 'C', 'R']);
        assert_eq!(stacks[9], vec!['L', 'D', 'T', 'R', 'H', 'P', 'F', 'S']);
    }

    #[test]
    fn test_can_parse_move() {
        assert_eq!(Move::from("move 8 from 7 to 1"), Move { count: 8, from: 7, to: 1 });
    }

    #[test]
    fn test_can_parse_input() {
        let result: Vec<Move> = read_input().iter()
            .map(|s| Move::from(s))
            .collect();
        assert_eq!(result[0], Move { count: 8, from: 7, to: 1 });
        assert_eq!(result[1], Move { count: 9, from: 1, to: 9 });
    }

    #[test]
    fn test_can_execute_move_1_a() {
        let mut stacks = create_stacks();
        Move { count: 0, from: 9, to: 1 }.execute_1(&mut stacks);
        assert_eq!(stacks[1], vec!['H', 'C', 'R']);
        assert_eq!(stacks[9], vec!['L', 'D', 'T', 'R', 'H', 'P', 'F', 'S']);
    }

    #[test]
    fn test_can_execute_move_1_b() {
        let mut stacks = create_stacks();
        Move { count: 2, from: 9, to: 1 }.execute_1(&mut stacks);
        assert_eq!(stacks[1], vec!['H', 'C', 'R', 'S', 'F']);
        assert_eq!(stacks[9], vec!['L', 'D', 'T', 'R', 'H', 'P']);
    }

    #[test]
    fn test_can_execute_move_1_c() {
        let mut stacks = create_stacks();
        Move { count: 3, from: 1, to: 6 }.execute_1(&mut stacks);
        assert_eq!(stacks[1].len(), 0);
        assert_eq!(stacks[6], vec!['T', 'H', 'C', 'G', 'R', 'C', 'H']);
    }

    #[test]
    fn test_can_execute_move_2_a() {
        let mut stacks = create_stacks();
        Move { count: 0, from: 9, to: 1 }.execute_2(&mut stacks);
        assert_eq!(stacks[1], vec!['H', 'C', 'R']);
        assert_eq!(stacks[9], vec!['L', 'D', 'T', 'R', 'H', 'P', 'F', 'S']);
    }

    #[test]
    fn test_can_execute_move_2_b() {
        let mut stacks = create_stacks();
        Move { count: 2, from: 9, to: 1 }.execute_2(&mut stacks);
        assert_eq!(stacks[1], vec!['H', 'C', 'R', 'F', 'S']);
        assert_eq!(stacks[9], vec!['L', 'D', 'T', 'R', 'H', 'P']);
    }

    #[test]
    fn test_can_execute_move_2_c() {
        let mut stacks = create_stacks();
        Move { count: 3, from: 1, to: 6 }.execute_2(&mut stacks);
        assert_eq!(stacks[1].len(), 0);
        assert_eq!(stacks[6], vec!['T', 'H', 'C', 'G', 'H', 'C', 'R']);
    }

    #[test]
    fn test_can_get_top_of_stacks() {
        let stacks = create_stacks();
        assert_eq!(get_top_of_stacks(&stacks), "RFQJBGLCS");
    }

    #[test]
    fn run_part_1_with_puzzle_input() {
        let mut stacks = create_stacks();
        read_input().iter()
            .map(|s| Move::from(s))
            .for_each(|mv| mv.execute_1(&mut stacks));
        let result = get_top_of_stacks(&stacks);
        assert_eq!(result, "SHQWSRBDL");
    }

    #[test]
    fn run_part_2_with_puzzle_input() {
        let mut stacks = create_stacks();
        read_input().iter()
            .map(|s| Move::from(s))
            .for_each(|mv| mv.execute_2(&mut stacks));
        let result = get_top_of_stacks(&stacks);
        assert_eq!(result, "CDTQZHBRS");
    }
}

fn main() {}
