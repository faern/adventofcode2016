extern crate base;

use base::{Part, ProblemSolver};
use base::geo::{Position, Direction};

use std::str::FromStr;

pub fn get_solver() -> Box<ProblemSolver> {
    Box::new(Day2)
}

struct Day2;

impl ProblemSolver for Day2 {
    fn solve(&self, part: Part, input: String) -> Result<String, String> {
        let movements = parse_input(input)?;
        match part {
            Part::One => enter_code(movements, Box::new(SaneKeyPad::new())),
            Part::Two => enter_code(movements, Box::new(CrazyKeyPad::new())),
        }
    }
}

fn parse_input(input: String) -> Result<Vec<Vec<Direction>>, String> {
    let mut movements = vec![];
    for line in input.split_terminator("\n") {
        let mut key_movements = vec![];
        for c in line.chars() {
            let direction = Direction::from_str(&c.to_string())?;
            key_movements.push(direction);
        }
        movements.push(key_movements);
    }
    Ok(movements)
}

fn enter_code(movements: Vec<Vec<Direction>>, mut keypad: Box<KeyPad>) -> Result<String, String> {
    let mut code = vec![];
    for one_digit_movements in movements {
        keypad.reset();
        for movement in one_digit_movements {
            keypad.walk(&movement);
        }
        code.push(keypad.key());
    }
    Ok(code.join(""))
}

trait KeyPad {
    fn active_position(&self) -> Position;
    fn set_active_position(&mut self, position: Position) -> Result<(), ()>;

    fn key(&self) -> String;

    fn walk(&mut self, direction: &Direction) {
        let mut new_active_key = self.active_position().clone();
        new_active_key.walk(direction, 1);
        self.set_active_position(new_active_key).unwrap_or(());
    }

    fn reset(&mut self);

    fn is_valid_position(&self, position: &Position) -> bool;
}

struct SaneKeyPad {
    active_key: Position,
}

impl SaneKeyPad {
    pub fn new() -> Self {
        SaneKeyPad {
            active_key: Self::initial_position(),
        }
    }

    fn initial_position() -> Position {
        Position(1, 1)
    }
}

impl KeyPad for SaneKeyPad {
    fn active_position(&self) -> Position {
        self.active_key
    }

    fn set_active_position(&mut self, position: Position) -> Result<(), ()> {
        if self.is_valid_position(&position) {
            self.active_key = position;
            Ok(())
        } else {
            Err(())
        }
    }

    fn key(&self) -> String {
        match self.active_key {
            Position(0, 2) => 1.to_string(),
            Position(1, 2) => 2.to_string(),
            Position(2, 2) => 3.to_string(),
            Position(0, 1) => 4.to_string(),
            Position(1, 1) => 5.to_string(),
            Position(2, 1) => 6.to_string(),
            Position(0, 0) => 7.to_string(),
            Position(1, 0) => 8.to_string(),
            Position(2, 0) => 9.to_string(),
            _ => unreachable!(),
        }
    }

    fn reset(&mut self) {
        self.active_key = Self::initial_position();
    }

    fn is_valid_position(&self, position: &Position) -> bool {
        !(position.0 < 0 || position.0 > 2 || position.1 < 0 || position.1 > 2)
    }
}

struct CrazyKeyPad {
    active_key: Position,
}

impl CrazyKeyPad {
    pub fn new() -> Self {
        CrazyKeyPad {
            active_key: Self::initial_position(),
        }
    }

    fn initial_position() -> Position {
        Position(-2, 0)
    }
}

impl KeyPad for CrazyKeyPad {
    fn active_position(&self) -> Position {
        self.active_key
    }

    fn set_active_position(&mut self, position: Position) -> Result<(), ()> {
        if self.is_valid_position(&position) {
            self.active_key = position;
            Ok(())
        } else {
            Err(())
        }
    }

    fn key(&self) -> String {
        match self.active_key {
            Position(0, 2) => "1".to_owned(),

            Position(-1, 1) => "2".to_owned(),
            Position(0, 1) => "3".to_owned(),
            Position(1, 1) => "4".to_owned(),

            Position(-2, 0) => "5".to_owned(),
            Position(-1, 0) => "6".to_owned(),
            Position(0, 0) => "7".to_owned(),
            Position(1, 0) => "8".to_owned(),
            Position(2, 0) => "9".to_owned(),

            Position(-1, -1) => "A".to_owned(),
            Position(0, -1) => "B".to_owned(),
            Position(1, -1) => "C".to_owned(),

            Position(0, -2) => "D".to_owned(),

            _ => unreachable!(),
        }
    }

    fn reset(&mut self) {
        self.active_key = Self::initial_position();
    }

    fn is_valid_position(&self, position: &Position) -> bool {
        (position.0.abs() + position.1.abs()) <= 2
    }
}

#[cfg(test)]
mod tests {
    use super::KeyPad;
    use base::geo::Direction;

    #[test]
    fn keypad_new() {
        let keypad = KeyPad::new();
        assert_eq!(5, keypad.key());
    }

    #[test]
    fn keypad_move() {
        let mut keypad = KeyPad::new();
        keypad.walk(&Direction::North);
        assert_eq!(2, keypad.key());
    }

    #[test]
    fn keypad_move_too_far() {
        let mut keypad = KeyPad::new();
        keypad.walk(&Direction::North);
        keypad.walk(&Direction::East);
        keypad.walk(&Direction::East);
        assert_eq!(3, keypad.key());
    }

    #[test]
    fn keypad_move_down_and_away() {
        let mut keypad = KeyPad::new();
        keypad.walk(&Direction::South);
        assert_eq!(8, keypad.key());
        keypad.walk(&Direction::South);
        assert_eq!(8, keypad.key());
        keypad.walk(&Direction::East);
        assert_eq!(9, keypad.key());
        keypad.walk(&Direction::West);
        keypad.walk(&Direction::West);
        assert_eq!(7, keypad.key());
        keypad.walk(&Direction::West);
        assert_eq!(7, keypad.key());
    }

    #[test]
    fn keypad_move_to_start() {
        let mut keypad = KeyPad::new();
        keypad.walk(&Direction::North);
        keypad.walk(&Direction::South);
        assert_eq!(5, keypad.key());
    }
}
