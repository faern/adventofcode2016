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
            Part::One => enter_code(movements, KeyPad::new(SaneKeyPadPositions)),
            Part::Two => enter_code(movements, KeyPad::new(CrazyKeyPadPositions)),
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

fn enter_code<P>(movements: Vec<Vec<Direction>>, mut keypad: KeyPad<P>) -> Result<String, String>
    where P: KeyPadPositions
{
    let mut code = vec![];
    for one_digit_movements in movements {
        for movement in one_digit_movements {
            keypad.walk(&movement);
        }
        code.push(keypad.key());
    }
    Ok(code.join(""))
}

struct KeyPad<P: KeyPadPositions> {
    positions: P,
    active_position: Position,
}

impl<P: KeyPadPositions> KeyPad<P> {
    pub fn new(positions: P) -> Self {
        KeyPad {
            positions: positions,
            active_position: P::initial_position(),
        }
    }

    pub fn walk(&mut self, direction: &Direction) {
        let mut new_active_position = self.active_position.clone();
        new_active_position.walk(direction, 1);
        if self.positions.key(&new_active_position).is_ok() {
            self.active_position = new_active_position;
        }
    }

    pub fn key(&self) -> String {
        self.positions.key(&self.active_position).unwrap()
    }

    pub fn reset(&mut self) {
        self.active_position = P::initial_position();
    }
}

trait KeyPadPositions {
    fn key(&self, position: &Position) -> Result<String, ()>;
    fn initial_position() -> Position;
}

struct SaneKeyPadPositions;

impl KeyPadPositions for SaneKeyPadPositions {
    fn key(&self, position: &Position) -> Result<String, ()> {
        match *position {
            Position(0, 2) => Ok(1.to_string()),
            Position(1, 2) => Ok(2.to_string()),
            Position(2, 2) => Ok(3.to_string()),
            Position(0, 1) => Ok(4.to_string()),
            Position(1, 1) => Ok(5.to_string()),
            Position(2, 1) => Ok(6.to_string()),
            Position(0, 0) => Ok(7.to_string()),
            Position(1, 0) => Ok(8.to_string()),
            Position(2, 0) => Ok(9.to_string()),
            _ => Err(()),
        }
    }

    fn initial_position() -> Position {
        Position(1, 1)
    }
}

struct CrazyKeyPadPositions;

impl KeyPadPositions for CrazyKeyPadPositions {
    fn key(&self, position: &Position) -> Result<String, ()> {
        match *position {
            Position(0, 2) => Ok("1".to_owned()),

            Position(-1, 1) => Ok("2".to_owned()),
            Position(0, 1) => Ok("3".to_owned()),
            Position(1, 1) => Ok("4".to_owned()),

            Position(-2, 0) => Ok("5".to_owned()),
            Position(-1, 0) => Ok("6".to_owned()),
            Position(0, 0) => Ok("7".to_owned()),
            Position(1, 0) => Ok("8".to_owned()),
            Position(2, 0) => Ok("9".to_owned()),

            Position(-1, -1) => Ok("A".to_owned()),
            Position(0, -1) => Ok("B".to_owned()),
            Position(1, -1) => Ok("C".to_owned()),

            Position(0, -2) => Ok("D".to_owned()),

            _ => Err(()),
        }
    }

    fn initial_position() -> Position {
        Position(-2, 0)
    }
}

#[cfg(test)]
mod tests {
    use base::geo::Direction;
    use super::{KeyPad, SaneKeyPadPositions};

    #[test]
    fn keypad_new() {
        let keypad = KeyPad::new(SaneKeyPadPositions);
        assert_eq!("5", keypad.key());
    }

    #[test]
    fn keypad_move() {
        let mut keypad = KeyPad::new(SaneKeyPadPositions);
        keypad.walk(&Direction::North);
        assert_eq!("2", keypad.key());
    }

    #[test]
    fn keypad_move_too_far() {
        let mut keypad = KeyPad::new(SaneKeyPadPositions);
        keypad.walk(&Direction::North);
        keypad.walk(&Direction::East);
        keypad.walk(&Direction::East);
        assert_eq!("3", keypad.key());
    }

    #[test]
    fn keypad_move_down_and_away() {
        let mut keypad = KeyPad::new(SaneKeyPadPositions);
        keypad.walk(&Direction::South);
        assert_eq!("8", keypad.key());
        keypad.walk(&Direction::South);
        assert_eq!("8", keypad.key());
        keypad.walk(&Direction::East);
        assert_eq!("9", keypad.key());
        keypad.walk(&Direction::West);
        keypad.walk(&Direction::West);
        assert_eq!("7", keypad.key());
        keypad.walk(&Direction::West);
        assert_eq!("7", keypad.key());
    }

    #[test]
    fn keypad_move_to_start() {
        let mut keypad = KeyPad::new(SaneKeyPadPositions);
        keypad.walk(&Direction::North);
        keypad.walk(&Direction::South);
        assert_eq!("5", keypad.key());
    }
}
