use std::collections::HashSet;
use aoc_2022::{read_vector_of_string};
use aoc_2022::geom::Point;

const ZERO_AS_U8: u8 = '0' as u8;

pub fn read_input() -> Vec<String> {
    read_vector_of_string("input/day08.txt")
}

pub fn to_matrix_of_u8(rows: &Vec<&str>) -> Vec<Vec<u8>> {
    rows.iter()
        .map(|row| to_vector_of_u8(row))
        .collect()
}

fn to_vector_of_u8(row: &str) -> Vec<u8> {
    row.chars()
        .map(|c| c as u8 - ZERO_AS_U8)
        .collect()
}

/// Returns the height of the tree at 'pos', which is assumed to inside the forest.
pub fn get_height(forest: &Vec<Vec<u8>>, pos: &Point) -> u8 {
    *forest
        .get(pos.y as usize)
        .expect("matrix has enough rows")
        .get(pos.x as usize)
        .expect("matrix has enough columns")
}

pub fn find_visible_trees(forest: &Vec<Vec<u8>>, start: Point, dx: i32, dy: i32) -> Vec<Point> {
    let mut visible_trees = Vec::new();

    let mut height_of_highest_so_far = -1;
    let mut pos = start;
    while is_in_forest(forest, &pos) {
        let height = get_height(&forest, &pos) as i32;
        if height > height_of_highest_so_far {
            visible_trees.push(pos.clone());
            height_of_highest_so_far = height;
        }
        pos = pos.translate(dx, dy);
    }

    visible_trees
}

/// Returns true if the given position is inside the forest.
fn is_in_forest(forest: &Vec<Vec<u8>>, pos: &Point) -> bool {
    forest.len() > pos.y as usize &&
        forest.len() > 0 &&
        forest.get(0).unwrap().len() > pos.x as usize
}

pub fn find_all_visible_trees(forest: &Vec<Vec<u8>>) -> HashSet<Point> {
    let mut visible_trees: HashSet<Point> = HashSet::new();

    let width = forest.get(0).unwrap().len();
    let height = forest.len();

    // Top
    for x in 0..width {
        let start = Point::of(x as i32, 0);
        let vec = find_visible_trees(&forest, start, 0, 1);
        for p in vec { visible_trees.insert(p); }
    }
    // Bottom
    for x in 0..width {
        let start = Point::of(x as i32, (height - 1) as i32);
        let vec = find_visible_trees(&forest, start, 0, -1);
        for p in vec { visible_trees.insert(p); }
    }
    // Left
    for y in 0..height {
        let start = Point::of(0, y as i32);
        let vec = find_visible_trees(&forest, start, 1, 0);
        for p in vec { visible_trees.insert(p); }
    }
    // Right
    for y in 0..height {
        let start = Point::of((width - 1) as i32, y as i32);
        let vec = find_visible_trees(&forest, start, -1, 0);
        for p in vec { visible_trees.insert(p); }
    }

    visible_trees
}

/// Returns the four viewing distances for position 'start'.
pub fn get_viewing_distances(forest: &Vec<Vec<u8>>, start: Point) -> Vec<i32> {
    vec![
        get_viewing_distance(forest, &start, 1, 0),
        get_viewing_distance(forest, &start, -1, 0),
        get_viewing_distance(forest, &start, 0, 1),
        get_viewing_distance(forest, &start, 0, -1),
    ]
}

/// Returns the viewing distance for the direction defined by dx and dy.
pub fn get_viewing_distance(forest: &Vec<Vec<u8>>, start: &Point, dx: i32, dy: i32) -> i32 {
    let my_height = get_height(forest, &start);

    let mut count = 0;

    let mut pos = start.translate(dx, dy);
    while is_in_forest(forest, &pos) {
        count += 1;
        if get_height(forest, &pos) >= my_height {
            break;
        }
        pos = pos.translate(dx, dy);
    }

    count
}

/// Calculates the scenic score by multiplying the different viewing distances.
pub fn calculate_scenic_score(viewing_distances: &Vec<i32>) -> i32 {
    viewing_distances.iter().fold(1, |a, b| a * b)
}

#[cfg(test)]
mod tests {
    use std::cmp::max;
    use std::collections::HashSet;
    use aoc_2022::to_vector_of_str;
    use super::*;

    #[test]
    fn test_can_read_input() {
        let input = read_input();
        assert_eq!(input[0], "102212110110302230012132130441442043243242145112422525112333344240121120342411110001231112222021211");
    }

    #[test]
    fn test_can_convert_to_matrix_of_int() {
        let input = vec!["123", "456", "789"];
        let result = to_matrix_of_u8(&input);
        assert_eq!(result, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    }

    #[test]
    fn test_can_get_height() {
        let forest = to_matrix_of_u8(&vec!["123", "456", "789"]);
        assert_eq!(get_height(&forest, &Point::of(0, 0)), 1);
        assert_eq!(get_height(&forest, &Point::of(1, 0)), 2);
        assert_eq!(get_height(&forest, &Point::of(2, 0)), 3);
        assert_eq!(get_height(&forest, &Point::of(0, 2)), 7);
        assert_eq!(get_height(&forest, &Point::of(1, 2)), 8);
        assert_eq!(get_height(&forest, &Point::of(2, 2)), 9);
    }

    #[test]
    fn test_is_in_forest() {
        let forest = to_matrix_of_u8(&vec!["123", "456", "789"]);

        assert_eq!(is_in_forest(&forest, &Point::of(0, 0)), true);
        assert_eq!(is_in_forest(&forest, &Point::of(1, 2)), true);
        assert_eq!(is_in_forest(&forest, &Point::of(2, 0)), true);

        assert_eq!(is_in_forest(&forest, &Point::of(3, 0)), false);
        assert_eq!(is_in_forest(&forest, &Point::of(2, 3)), false);
    }

    #[test]
    fn test_can_find_visible_trees() {
        let forest = to_matrix_of_u8(&vec![
            "123",
            "645",
            "709",
        ]);

        assert_eq!(find_visible_trees(&forest, Point::of(0, 0), 1, 0),
                   vec![Point::of(0, 0), Point::of(1, 0), Point::of(2, 0)]);
        assert_eq!(find_visible_trees(&forest, Point::of(2, 0), -1, 0),
                   vec![Point::of(2, 0)]);
        assert_eq!(find_visible_trees(&forest, Point::of(1, 0), 0, 1),
                   vec![Point::of(1, 0), Point::of(1, 1)]);
        assert_eq!(find_visible_trees(&forest, Point::of(1, 2), 0, -1),
                   vec![Point::of(1, 2), Point::of(1, 1)]);
    }

    #[test]
    fn test_can_find_all_visible_trees_1() {
        let forest = to_matrix_of_u8(&vec![
            "123",
            "645",
            "709",
        ]);

        let expected: HashSet<Point> = HashSet::from([
            Point::of(0, 0),
            Point::of(0, 1),
            Point::of(0, 2),
            Point::of(1, 0),
            Point::of(1, 1),
            Point::of(1, 2),
            Point::of(2, 0),
            Point::of(2, 1),
            Point::of(2, 2),
        ]);

        assert_eq!(find_all_visible_trees(&forest), expected);
    }

    #[test]
    fn test_can_find_all_visible_trees_2() {
        let forest = to_matrix_of_u8(&vec![
            "123",
            "605",
            "709",
        ]);

        let expected: HashSet<Point> = HashSet::from([
            Point::of(0, 0),
            Point::of(0, 1),
            Point::of(0, 2),
            Point::of(1, 0),
            //Point::of(1, 1),
            Point::of(1, 2),
            Point::of(2, 0),
            Point::of(2, 1),
            Point::of(2, 2),
        ]);

        assert_eq!(find_all_visible_trees(&forest), expected);
    }

    #[test]
    fn test_with_example_part_1() {
        let forest = to_matrix_of_u8(&vec![
            "30373",
            "25512",
            "65332",
            "33549",
            "35390",
        ]);

        let expected: HashSet<Point> = HashSet::from([
            Point::of(0, 0),
            Point::of(1, 0),
            Point::of(2, 0),
            Point::of(3, 0),
            Point::of(4, 0),
            Point::of(0, 1),
            Point::of(1, 1),
            Point::of(2, 1),
            //Point::of(3, 1),
            Point::of(4, 1),
            Point::of(0, 2),
            Point::of(1, 2),
            //Point::of(2, 2),
            Point::of(3, 2),
            Point::of(4, 2),
            Point::of(0, 3),
            //Point::of(1, 3),
            Point::of(2, 3),
            //Point::of(3, 3),
            Point::of(4, 3),
            Point::of(0, 4),
            Point::of(1, 4),
            Point::of(2, 4),
            Point::of(3, 4),
            Point::of(4, 4),
        ]);

        assert_eq!(find_all_visible_trees(&forest), expected);
        assert_eq!(find_all_visible_trees(&forest).len(), 21);
    }

    #[test]
    fn test_calculate_scenic_score() {
        assert_eq!(calculate_scenic_score(&vec![1, 1, 2, 2]), 4);
        assert_eq!(calculate_scenic_score(&vec![2, 2, 1, 2]), 8);
        assert_eq!(calculate_scenic_score(&vec![-1, 5, 7]), -35);
    }

    #[test]
    fn test_with_example_part_2() {
        let forest = to_matrix_of_u8(&vec![
            "30373",
            "25512",
            "65332",
            "33549",
            "35390",
        ]);

        let viewing_distances = get_viewing_distances(&forest, Point::of(2, 1));
        assert_eq!(calculate_scenic_score(&viewing_distances), 4);
        let viewing_distances = get_viewing_distances(&forest, Point::of(2, 3));
        assert_eq!(calculate_scenic_score(&viewing_distances), 8);
        let viewing_distances = get_viewing_distances(&forest, Point::of(4, 3));
        assert_eq!(calculate_scenic_score(&viewing_distances), 0);
    }

    #[test]
    fn run_part_1_with_puzzle_input() {
        let input = read_input();
        let forest = to_matrix_of_u8(&to_vector_of_str(&input));
        let visible = find_all_visible_trees(&forest);
        assert_eq!(visible.len(), 1843);
    }

    #[test]
    fn run_part_2_with_puzzle_input() {
        let input = read_input();
        let forest = to_matrix_of_u8(&to_vector_of_str(&input));

        let width = forest.get(0).unwrap().len();
        let height = forest.len();

        let mut max_score = 0;
        for x in 0..width {
            for y in 0..height {
                let start = Point::of(x as i32, y as i32);
                let distances = get_viewing_distances(&forest, start);
                let score = calculate_scenic_score(&distances);
                max_score = max(score, max_score);
            }
        }

        assert_eq!(max_score, 180_000);
    }
}

fn main() {}
