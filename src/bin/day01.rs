use aoc_2022::read_blank_separated_matrix_of_i32;

pub fn read_input() -> Vec<Vec<i32>> {
    read_blank_separated_matrix_of_i32("input/day01.txt")
}

#[cfg(test)]
mod tests {
    use aoc_2022::sum_rows;
    use super::*;

    #[test]
    fn test_can_read_input() {
        let result = read_input();
        assert_eq!(result[0], vec![18313, 2404, 10479]);
    }

    #[test]
    fn run_part_1_with_puzzle_input() {
        let calories_per_elf = sum_rows(&read_input());
        let max_calories = calories_per_elf.iter()
            .max()
            .expect("Empty vector");
        assert_eq!(*max_calories, 71924);
    }

    #[test]
    fn run_part_2_with_puzzle_input() {
        let mut calories_per_elf = sum_rows(&read_input());
        calories_per_elf.sort();
        let sum_calories: i32 = calories_per_elf.iter()
            .rev()
            .take(3)
            .sum();
        assert_eq!(sum_calories, 210406);
    }
}

fn main() {}
