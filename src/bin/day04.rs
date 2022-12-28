use aoc_2022::{read_matrix_of_string, to_i32};

#[derive(Debug, PartialEq)]
pub struct Interval {
    from: i32,
    to: i32,
}

impl Interval {
    pub fn from(s: &str) -> Self {
        let v: Vec<i32> = s.split("-").map(to_i32).collect();
        Interval {
            from: v[0],
            to: v[1],
        }
    }

    pub fn contains(&self, other: &Interval) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    pub fn overlaps(&self, other: &Interval) -> bool {
        self.from <= other.to && self.to >= other.from
    }
}

pub fn to_intervals(strings: &Vec<String>) -> Vec<Interval> {
    strings.iter().map(|s| Interval::from(s)).collect::<Vec<Interval>>()
}

pub fn read_input() -> Vec<Vec<String>> {
    read_matrix_of_string("input/day04.txt", ",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_read_input() {
        let result = read_input();
        assert_eq!(result[0], vec!["98-99", "3-97"]);
    }

    #[test]
    fn test_can_create_interval() {
        assert_eq!(Interval::from("7-345"), Interval { from: 7, to: 345 });
        assert_eq!(Interval::from("100-100"), Interval { from: 100, to: 100 });
    }

    #[test]
    fn test_contains() {
        assert_eq!(Interval { from: 7, to: 15 }.contains(&Interval { from: 7, to: 14 }), true);
        assert_eq!(Interval { from: 1, to: 8 }.contains(&Interval { from: 7, to: 8 }), true);
        assert_eq!(Interval { from: 1, to: 8 }.contains(&Interval { from: 7, to: 9 }), false);
        assert_eq!(Interval { from: 23, to: 88 }.contains(&Interval { from: 90, to: 999 }), false);
    }

    #[test]
    fn test_overlaps() {
        assert_eq!(Interval { from: 7, to: 15 }.overlaps(&Interval { from: 7, to: 14 }), true);
        assert_eq!(Interval { from: 1, to: 8 }.overlaps(&Interval { from: 7, to: 8 }), true);
        assert_eq!(Interval { from: 4, to: 5 }.overlaps(&Interval { from: 3, to: 8 }), true);
        assert_eq!(Interval { from: 1, to: 8 }.overlaps(&Interval { from: 7, to: 9 }), true);
        assert_eq!(Interval { from: 4, to: 28 }.overlaps(&Interval { from: 28, to: 38 }), true);
        assert_eq!(Interval { from: 4, to: 28 }.overlaps(&Interval { from: 1, to: 5 }), true);
        assert_eq!(Interval { from: 23, to: 88 }.overlaps(&Interval { from: 90, to: 999 }), false);
        assert_eq!(Interval { from: 15, to: 17 }.overlaps(&Interval { from: 3, to: 14 }), false);
        assert_eq!(Interval { from: 159, to: 217 }.overlaps(&Interval { from: 158, to: 158 }), false);
    }

    #[test]
    fn run_part_1_with_puzzle_input() {
        let count = read_input().iter()
            .map(to_intervals)
            .filter(|i| i[0].contains(&i[1]) || i[1].contains(&i[0]))
            .count();
        assert_eq!(count, 459);
    }

    #[test]
    fn run_part_2_with_puzzle_input() {
        let count = read_input().iter()
            .map(to_intervals)
            .filter(|i| i[0].overlaps(&i[1]))
            .count();
        assert_eq!(count, 779);
    }
}

fn main() {}
