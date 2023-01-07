use crate::geom::Direction::{D, L, R, U};
use crate::to_i32;

/// Returns the sign of 'value': -1, 0, or 1.
pub const fn sgn(value: i32) -> i32 {
    if value < 0 { -1 } else if value > 0 { 1 } else { 0 }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Direction {
    U,
    D,
    L,
    R,
}

impl Direction {
    /// Creates a new Direction from a single character, e.g. 'D'.
    pub fn from(c: char) -> Self {
        match c {
            'U' => Some(U),
            'D' => Some(D),
            'L' => Some(L),
            'R' => Some(R),
            _ => None
        }.expect(&*format!("Direction is U, D, L, or R: {}", c))
    }

    pub const fn dx(&self) -> i32 {
        match self {
            L => -1,
            R => 1,
            _ => 0,
        }
    }

    pub const fn dy(&self) -> i32 {
        match self {
            U => 1,
            D => -1,
            _ => 0,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Move {
    pub dx: i32,
    pub dy: i32,
}

impl Move {
    pub const fn of(dx: i32, dy: i32) -> Self {
        Move { dx, dy }
    }

    /// Creates a new Move from a direction and a distance.
    pub const fn from_dir(direction: Direction, distance: i32) -> Self {
        Move { dx: direction.dx() * distance, dy: direction.dy() * distance }
    }

    /// Creates a new Move from a string like "L 7".
    pub fn from_str(text: &str) -> Self {
        let parts: Vec<&str> = text.split_ascii_whitespace().collect();
        Move::from_dir(
            Direction::from(parts[0].chars().nth(0).expect("String is not empty")),
            to_i32(parts[1]),
        )
    }

    /// Splits this Move into a series of horizontal and vertical single-step moves.
    pub fn split(&self) -> Vec<Move> {
        let mut steps: Vec<Move> = Vec::new();
        
        let mut dx = self.dx;
        while dx != 0 {
            steps.push(Move::of(sgn(dx), 0));
            dx -= sgn(dx);
        }
        
        let mut dy = self.dy;
        while dy != 0 {
            steps.push(Move::of(0, sgn(dy)));
            dy -= sgn(dy);
        }
        
        steps
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Creates a new point from x and y.
    pub const fn of(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    /// Returns a new point that is the result of moving this point
    /// in the direction defined by dx and dy.
    pub const fn translate(&self, dx: i32, dy: i32) -> Point {
        Point::of(self.x + dx, self.y + dy)
    }

    /// Returns a new point that is the result of moving this point
    /// as defined by the given Move.
    pub const fn translate_by_move(&self, mv: &Move) -> Point {
        self.translate(mv.dx, mv.dy)
    }

    /// Returns true if the other point is touching this point,
    /// vertically, horizontally, or diagonally. If the points
    /// overlap, they are also touching.
    pub const fn touches(&self, other: &Point) -> bool {
        self.x >= other.x - 1 && self.x <= other.x + 1 &&
            self.y >= other.y - 1 && self.y <= other.y + 1
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    const MV_1_0: Move = Move::of(1, 0);
    const MV_0_1: Move = Move::of(0, 1);
    const MV_0_N1: Move = Move::of(0, -1);

    #[test]
    fn test_can_calculate_sgn() {
        assert_eq!(sgn(-123_456), -1);
        assert_eq!(sgn(-1), -1);
        assert_eq!(sgn(0), 0);
        assert_eq!(sgn(1), 1);
        assert_eq!(sgn(9_999_999), 1);
    }

    #[test]
    fn test_can_create_move() {
        assert_eq!(Move::from_dir(D, 7), Move { dx: 0, dy: -7 });
        assert_eq!(Move::from_dir(R, 0), Move { dx: 0, dy: 0 });
    }

    #[test]
    fn test_can_create_move_from_str() {
        assert_eq!(Move::from_str("L 5"), Move { dx: -5, dy: 0 });
        assert_eq!(Move::from_str("U 99"), Move { dx: 0, dy: 99 });
    }

    #[test]
    fn test_can_translate_point() {
        assert_eq!(Point::of(0, 0).translate(5, 3), Point::of(5, 3));
        assert_eq!(Point::of(17, 47).translate(-2, 0), Point::of(15, 47));
    }

    #[test]
    fn test_can_translate_point_by_move() {
        assert_eq!(Point::of(0, 0).translate_by_move(&Move::of(5, 3)), Point::of(5, 3));
        assert_eq!(Point::of(17, 47).translate_by_move(&Move::of(-2, 0)), Point::of(15, 47));
        assert_eq!(Point::of(10, -3).translate_by_move(&Move::of(-10, 3)), Point::of(0, 0));
    }

    #[test]
    fn test_can_split_move() {
        let steps = Move::of(0, 0).split();
        assert_eq!(steps, vec![]);

        let steps = Move::of(1, 0).split();
        assert_eq!(steps, vec![MV_1_0]);

        let steps = Move::of(0, -1).split();
        assert_eq!(steps, vec![MV_0_N1]);

        let steps = Move::of(2, -1).split();
        assert_eq!(steps, vec![MV_1_0, MV_1_0, MV_0_N1]);

        let steps = Move::of(2, 3).split();
        assert_eq!(steps, vec![MV_1_0, MV_1_0, MV_0_1, MV_0_1, MV_0_1]);
    }

    #[test]
    fn test_point_touches() {
        assert_eq!(Point::of(5, 8).touches(&Point::of(4, 8)), true);
        assert_eq!(Point::of(5, 8).touches(&Point::of(6, 8)), true);
        assert_eq!(Point::of(5, 8).touches(&Point::of(5, 7)), true);
        assert_eq!(Point::of(5, 8).touches(&Point::of(5, 9)), true);

        assert_eq!(Point::of(-6, 0).touches(&Point::of(-5, -1)), true);
        assert_eq!(Point::of(-6, 0).touches(&Point::of(-7, -1)), true);
        assert_eq!(Point::of(-6, 0).touches(&Point::of(-5, 1)), true);
        assert_eq!(Point::of(-6, 0).touches(&Point::of(-7, 1)), true);

        assert_eq!(Point::of(17, 18).touches(&Point::of(17, 18)), true);

        assert_eq!(Point::of(17, 18).touches(&Point::of(17, 20)), false);
        assert_eq!(Point::of(17, 18).touches(&Point::of(15, 18)), false);
        assert_eq!(Point::of(17, 18).touches(&Point::of(-17, 18)), false);
    }

    #[test]
    fn test_can_hash_point() {
        let mut set: HashSet<Point> = HashSet::new();
        set.insert(Point::of(0, 0));
        set.insert(Point::of(1, 0));
        set.insert(Point::of(0, 1));
        set.insert(Point::of(1, 1));
        set.insert(Point::of(0, 0));
        set.insert(Point::of(1, 0));
        set.insert(Point::of(0, 1));
        set.insert(Point::of(1, 1));
        assert_eq!(set.len(), 4);
    }
}
