use aoc_2022::{read_vector_of_string};
use crate::Move::{Paper, Rock, Scissors};
use crate::Result::{Draw, Lose, Win};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from(c: char) -> Move {
        match c {
            'A' | 'X' => Some(Rock),
            'B' | 'Y' => Some(Paper),
            'C' | 'Z' => Some(Scissors),
            _ => None
        }.expect(&*format!("Illegal move: {}", c))
    }

    fn score(&self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn compete(&self, other: &Move) -> Result {
        match self {
            Rock => if *other == Paper { Lose } else if *other == Scissors { Win } else { Draw },
            Paper => if *other == Scissors { Lose } else if *other == Rock { Win } else { Draw },
            Scissors => if *other == Rock { Lose } else if *other == Paper { Win } else { Draw },
        }
    }

    /// Return the move that wins against self.
    fn get_winning_move(&self) -> Move {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    /// Return the move that loses against self.
    fn get_losing_move(&self) -> Move {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Result {
    Win,
    Draw,
    Lose,
}

impl Result {
    fn score(&self) -> i32 {
        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Round {
    mv: Move,
    rs: char, // rs is either a move (part 1) or tells us how to choose a move (part 2)
}

impl Round {
    pub fn from(line: &str) -> Self {
        Round {
            mv: Move::from(line.chars().nth(0).expect("Invalid input")),
            rs: line.chars().nth(2).expect("Invalid input"),
        }
    }

    /// Use the move already stored in self.rs and calculate score for that move
    /// (Part 1)
    pub fn use_move_and_score(&self) -> i32 {
        let mv = Move::from(self.rs);
        mv.compete(&self.mv).score() + mv.score()
    }

    /// Choose a move using rules specified by self.rs and calculate score for that move
    /// (Part 2)
    pub fn choose_move_and_score(&self) -> i32 {
        let mv = self.choose_move();
        mv.compete(&self.mv).score() + mv.score()
    }

    /// Choose a move using the rules specified by self.rs
    /// X = loose
    /// Y = draw
    /// Z = win
    fn choose_move(&self) -> Move {
        if self.rs == 'X' { self.mv.get_losing_move() }
        else if self.rs == 'Z' { self.mv.get_winning_move() }
        else { self.mv.clone() }
    }
}

pub fn read_input() -> Vec<String> {
    read_vector_of_string("input/day02.txt")
}

pub fn read_rounds() -> Vec<Round> {
    read_input().iter()
        .map(|line| Round::from(line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_read_input() {
        let result = read_input();
        assert_eq!(result[0], "B Z");
    }

    #[test]
    fn test_can_read_rounds() {
        let result = read_rounds();
        assert_eq!(result[0], Round { mv: Paper, rs: 'Z' });
    }

    #[test]
    fn test_can_use_move_and_score() {
        assert_eq!(Round { mv: Rock, rs: 'Y' }.use_move_and_score(), 8);
        assert_eq!(Round { mv: Paper, rs: 'X' }.use_move_and_score(), 1);
        assert_eq!(Round { mv: Scissors, rs: 'Z' }.use_move_and_score(), 6);
    }

    #[test]
    fn test_can_choose_move_and_score() {
        assert_eq!(Round { mv: Rock, rs: 'Y' }.choose_move_and_score(), 4);
        assert_eq!(Round { mv: Paper, rs: 'X' }.choose_move_and_score(), 1);
        assert_eq!(Round { mv: Scissors, rs: 'Z' }.choose_move_and_score(), 7);
    }

    #[test]
    fn run_part_1_with_puzzle_input() {
        let result: i32 = read_rounds().iter()
            .map(Round::use_move_and_score)
            .sum();
        assert_eq!(result, 11449);
    }

    #[test]
    fn run_part_2_with_puzzle_input() {
        let result: i32 = read_rounds().iter()
            .map(Round::choose_move_and_score)
            .sum();
        assert_eq!(result, 13187);
    }
}

fn main() {}
