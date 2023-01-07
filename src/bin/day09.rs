use aoc_2022::geom::{Move, Point};
use aoc_2022::read_vector_of_string;
use std::collections::HashSet;

const ORIGO: Point = Point::of(0, 0);

pub fn read_input() -> Vec<String> {
    read_vector_of_string("input/day09.txt")
}

pub fn parse_moves(moves: &Vec<&str>) -> Vec<Move> {
    moves.iter().map(|s| Move::from_str(s)).collect()
}

pub const fn calculate_tail_move(head: &Point, tail: &Point) -> Option<Move> {
    if tail.touches(head) {
        return None;
    }
    // If the head is on the same row or column as the tail,
    // move closer along that row or column
    // If the head is neither on the same row, nor or the same column as the tail,
    // move closer diagonally
    let dx = if head.x > tail.x {
        1
    } else if head.x < tail.x {
        -1
    } else {
        0
    };
    let dy = if head.y > tail.y {
        1
    } else if head.y < tail.y {
        -1
    } else {
        0
    };
    Some(Move::of(dx, dy))
}

/// Moves the head and tail by first moving the head, and then the tail follows.
/// This function assumes the 'head_move' is a single-step move.
pub fn move_head_and_tail(head: &Point, tail: &Point, head_move: &Move) -> (Point, Point) {
    let new_head = head.translate_by_move(head_move);
    let tail_move = calculate_tail_move(&new_head, tail);
    let new_tail = match tail_move {
        Some(mv) => tail.translate_by_move(&mv),
        None => *tail,
    };
    (new_head, new_tail)
}

//noinspection RsBorrowChecker
pub fn make_moves_and_count_tail_positions(moves: &Vec<Move>) -> i32 {
    let mut head = ORIGO;
    let mut tail = ORIGO;

    let mut tail_positions: HashSet<Point> = HashSet::new();
    tail_positions.insert(tail);

    for mv in moves {
        for smv in mv.split() {
            // IntelliJ complains about this line, Cargo allows it
            (head, tail) = move_head_and_tail(&head, &tail, &smv);
            tail_positions.insert(tail);
        }
    }

    tail_positions.len() as i32
}

/// Moves the entire rope by first moving the head according to 'head_move',
/// and then the rest of the rope follows. This function assumes the 'head_move'
/// is a single-step move.
pub fn move_rope(rope: &Vec<Point>, head_move: &Move) -> Vec<Point> {
    let mut new_rope: Vec<Point> = Vec::new();

    // Move and add head
    let mut segment_pos = rope[0].translate_by_move(head_move);
    new_rope.push(segment_pos);

    for i in 1..rope.len() {
        let optional_move = calculate_tail_move(&segment_pos, &rope[i]);
        if optional_move == None {
            // Add the rest of the segments unchanged
            rope[i..].iter().for_each(|&p| new_rope.push(p));
            break;
        }

        // Move and add segment
        segment_pos = rope[i].translate_by_move(&optional_move.unwrap());
        new_rope.push(segment_pos);
    }

    new_rope
}

pub fn make_moves_and_count_rope_end_positions(moves: &Vec<Move>, rope_len: usize) -> i32 {
    let mut rope = vec![ORIGO; rope_len];

    let mut tail_positions: HashSet<Point> = HashSet::new();
    tail_positions.insert(rope[rope.len() - 1]);

    for mv in moves {
        for smv in mv.split() {
            rope = move_rope(&rope, &smv);
            tail_positions.insert(rope[rope.len() - 1]);
        }
    }

    tail_positions.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_2022::geom::Direction::{D, L, R, U};
    use aoc_2022::to_vector_of_str;

    #[test]
    fn test_can_read_input() {
        let input = read_input();
        let vec = to_vector_of_str(&input);
        assert_eq!(vec[0], "D 2");
        assert_eq!(vec[1], "L 2");
        assert_eq!(vec[2], "U 1");
        let moves = parse_moves(&vec);
        assert_eq!(moves[0], Move::of(0, -2));
        assert_eq!(moves[1], Move::of(-2, 0));
        assert_eq!(moves[2], Move::of(0, 1));
    }

    #[test]
    fn test_can_parse_moves() {
        let input = vec!["U 1", "D 75", "L 22", "R 0"];
        let moves = parse_moves(&input);
        assert_eq!(
            moves,
            vec![
                Move::from_dir(U, 1),
                Move::from_dir(D, 75),
                Move::from_dir(L, 22),
                Move::from_dir(R, 0),
            ]
        );
    }

    #[test]
    fn test_can_calculate_tail_move() {
        // Overlapping
        assert_eq!(
            calculate_tail_move(&Point::of(5, 9), &Point::of(5, 9)),
            None
        );
        // Touching
        assert_eq!(
            calculate_tail_move(&Point::of(5, 9), &Point::of(6, 9)),
            None
        );
        assert_eq!(
            calculate_tail_move(&Point::of(5, 9), &Point::of(6, 8)),
            None
        );
        assert_eq!(
            calculate_tail_move(&Point::of(5, 9), &Point::of(4, 10)),
            None
        );
        // Head is two steps up from tail
        assert_eq!(
            calculate_tail_move(&Point::of(5, 9), &Point::of(5, 7)),
            Some(Move::of(0, 1))
        );
        // Head is two steps down from tail
        assert_eq!(
            calculate_tail_move(&Point::of(5, 9), &Point::of(5, 11)),
            Some(Move::of(0, -1))
        );
        // Head is two steps left from tail
        assert_eq!(
            calculate_tail_move(&Point::of(5, 9), &Point::of(7, 9)),
            Some(Move::of(-1, 0))
        );
        // Head is two steps right from tail
        assert_eq!(
            calculate_tail_move(&Point::of(5, 9), &Point::of(3, 9)),
            Some(Move::of(1, 0))
        );
        // Head is two steps right and one step up
        assert_eq!(
            calculate_tail_move(&Point::of(5, 9), &Point::of(3, 8)),
            Some(Move::of(1, 1))
        );
        // Head is two steps up and one step left
        assert_eq!(
            calculate_tail_move(&Point::of(5, 9), &Point::of(6, 7)),
            Some(Move::of(-1, 1))
        );
    }

    #[test]
    fn test_can_move_head_and_tail() {
        // Overlapping - tail does not move
        let (head, tail) = move_head_and_tail(&Point::of(5, 9), &Point::of(5, 9), &Move::of(-1, 0));
        assert_eq!((head, tail), (Point::of(4, 9), Point::of(5, 9)));

        // Adjacent - head moves left - tail follows
        let (head, tail) = move_head_and_tail(&Point::of(5, 9), &Point::of(6, 9), &Move::of(-1, 0));
        assert_eq!((head, tail), (Point::of(4, 9), Point::of(5, 9)));

        // Adjacent - head moves down - tail follows
        let (head, tail) =
            move_head_and_tail(&Point::of(5, 9), &Point::of(5, 10), &Move::of(0, -1));
        assert_eq!((head, tail), (Point::of(5, 8), Point::of(5, 9)));

        // Diagonal - head moves up - tail is still touching
        let (head, tail) = move_head_and_tail(&Point::of(5, 9), &Point::of(6, 10), &Move::of(0, 1));
        assert_eq!((head, tail), (Point::of(5, 10), Point::of(6, 10)));

        // Diagonal - head moves left - tail moves diagonally
        let (head, tail) =
            move_head_and_tail(&Point::of(5, 9), &Point::of(6, 10), &Move::of(-1, 0));
        assert_eq!((head, tail), (Point::of(4, 9), Point::of(5, 9)));

        // Diagonal - head moves down - tail moves diagonally
        let (head, tail) =
            move_head_and_tail(&Point::of(5, 9), &Point::of(6, 10), &Move::of(0, -1));
        assert_eq!((head, tail), (Point::of(5, 8), Point::of(5, 9)));
    }

    #[test]
    fn test_move_and_count_single_complex_move() {
        let input = vec!["U 4"];
        let moves = parse_moves(&input);
        let count = make_moves_and_count_tail_positions(&moves);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_move_and_count_moves_right_and_left() {
        let input = vec!["R 3", "L 5"];
        let moves = parse_moves(&input);
        let count = make_moves_and_count_tail_positions(&moves);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_move_and_count_moves_down_and_left() {
        let input = vec!["D 7", "L 5"];
        let moves = parse_moves(&input);
        let count = make_moves_and_count_tail_positions(&moves);
        assert_eq!(count, 11);
    }

    #[test]
    fn test_move_and_count_moves_down_left_up() {
        let input = vec!["D 4", "L 5", "U 4"];
        let moves = parse_moves(&input);
        let count = make_moves_and_count_tail_positions(&moves);
        assert_eq!(count, 11);
    }

    #[test]
    fn test_move_and_count_moves_that_loop() {
        let input = vec!["D 3", "L 3", "U 2", "R 5"];
        let moves = parse_moves(&input);
        let count = make_moves_and_count_tail_positions(&moves);
        assert_eq!(count, 9);
    }

    const M_1_0: Move = Move::of(1, 0);
    const M_0_1: Move = Move::of(0, 1);
    const P_1_0: Point = Point::of(1, 0);
    const P_2_0: Point = Point::of(2, 0);
    const P_2_1: Point = Point::of(2, 1);
    const P_3_0: Point = Point::of(3, 0);
    const P_3_1: Point = Point::of(3, 1);
    const P_3_2: Point = Point::of(3, 2);

    #[test]
    fn test_can_move_rope() {
        let rope = vec![ORIGO, ORIGO, ORIGO];
        let new_rope = move_rope(&rope, &M_1_0);
        assert_eq!(new_rope, vec![P_1_0, ORIGO, ORIGO]);

        let rope = new_rope;
        let new_rope = move_rope(&rope, &M_1_0);
        assert_eq!(new_rope, vec![P_2_0, P_1_0, ORIGO]);

        let rope = new_rope;
        let new_rope = move_rope(&rope, &M_1_0);
        assert_eq!(new_rope, vec![P_3_0, P_2_0, P_1_0]);

        let rope = new_rope;
        let new_rope = move_rope(&rope, &M_0_1);
        assert_eq!(new_rope, vec![P_3_1, P_2_0, P_1_0]);

        let rope = new_rope;
        let new_rope = move_rope(&rope, &M_0_1);
        assert_eq!(new_rope, vec![P_3_2, P_3_1, P_2_1]);
    }

    #[test]
    fn test_move_rope_and_count_single_complex_move() {
        let input = vec!["U 4"];
        let moves = parse_moves(&input);
        let count = make_moves_and_count_rope_end_positions(&moves, 3);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_move_rope_and_count_down_and_left() {
        let input = vec!["D 3", "L 5"];
        let moves = parse_moves(&input);
        let count = make_moves_and_count_rope_end_positions(&moves, 3);
        assert_eq!(count, 5);
    }

    #[test]
    fn run_part_1_with_puzzle_input_using_rope_of_length_2() {
        let input = read_input();
        let vec = to_vector_of_str(&input);
        let moves = parse_moves(&vec);
        let count = make_moves_and_count_rope_end_positions(&moves, 2);
        assert_eq!(count, 6098);
    }

    #[test]
    fn run_part_1_with_puzzle_input() {
        let input = read_input();
        let vec = to_vector_of_str(&input);
        let moves = parse_moves(&vec);
        let count = make_moves_and_count_tail_positions(&moves);
        assert_eq!(count, 6098);
    }

    #[test]
    fn run_part_2_with_puzzle_input() {
        let input = read_input();
        let vec = to_vector_of_str(&input);
        let moves = parse_moves(&vec);
        let count = make_moves_and_count_rope_end_positions(&moves, 10);
        assert_eq!(count, 2597);
    }
}

fn main() {}
