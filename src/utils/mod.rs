// use bevy::prelude::*;

use crate::position::Position;

pub struct Raycast {
    current: Position,
    end: Position,
    x_dir: i32,
    y_dir: i32,
    error: i32,
    error_correct: Position,
}

impl Raycast {
    fn new(start: Position, end: Position) -> Self {
        let x_dir = if end.x > start.x { 1 } else { -1 };
        let y_dir = if end.y > start.y { 1 } else { -1 };

        let delta = (start - end).abs();
        let error = delta.x - delta.y;
        let error_correct = delta * 2;
        let current = start;
        Raycast {
            current,
            end,
            x_dir,
            y_dir,
            error,
            error_correct,
        }
    }
}

impl Iterator for Raycast {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        if self.current == self.end {
            return None;
        }

        if self.error > 0 {
            self.current = Position::new(self.current.x + self.x_dir, self.current.y);
            self.error -= self.error_correct.y;
        } else if self.error < 0 {
            self.current = Position::new(self.current.x, self.current.y + self.y_dir);
            self.error += self.error_correct.x;
        } else {
            self.current = Position::new(self.current.x + self.x_dir, self.current.y + self.y_dir);
        }

        Some(self.current)
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::tower::tower::Tower;

    #[test]
    fn test_raycast() {
        let level_name = "test_level";
        let map_name = "test_map";
        let mut tower = Tower::new();
        tower.insert_map_from_str(
            level_name,
            map_name,
            5,
            5,
            "
        | | | | | |
        | | | | | |
        | |W| | | |
        | | | |W| |
        | | | | | |
            ",
        );
        let tests: Vec<(Position, Position, bool)> = vec![
            // Horizontal Positive
            (Position::new(0, 0), Position::new(4, 0), true),
            (Position::new(0, 1), Position::new(4, 1), false),
            // Horizontal Negative
            (Position::new(4, 0), Position::new(2, 0), true),
            (Position::new(4, 1), Position::new(2, 1), false),
            // Vertical Positive
            (Position::new(0, 0), Position::new(0, 4), true),
            (Position::new(1, 0), Position::new(1, 4), false),
            // Vertical Negative
            (Position::new(0, 4), Position::new(0, 1), true),
            (Position::new(1, 4), Position::new(1, 1), false),
            // Diagonal Positive
            (Position::new(0, 0), Position::new(4, 4), true),
            (Position::new(0, 1), Position::new(3, 4), false),
            // Diagonal Negative
            (Position::new(1, 4), Position::new(4, 1), true),
            (Position::new(0, 4), Position::new(4, 0), false),
        ];
        for (start, end, result) in tests {
            let rc = Raycast::new(start, end);
            let mut visible: HashSet<Position> = HashSet::new();
            for pos in rc {
                visible.insert(pos);
                if !tower
                    .get_tile(level_name, map_name, &pos)
                    .unwrap()
                    .transparent
                {
                    break;
                }
            }
            assert_eq!(visible.contains(&end), result);
        }
    }
}
