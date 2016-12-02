use std::str::FromStr;


#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Turn {
    Right,
    Left,
}

impl FromStr for Turn {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Turn::Right),
            "L" => Ok(Turn::Left),
            _ => Err(format!("Invalid direction: {}", s)),
        }
    }
}

pub struct Step {
    turn: Turn,
    distance: i32,
}

impl Step {
    pub fn turn(&self) -> Turn {
        self.turn
    }

    pub fn distance(&self) -> i32 {
        self.distance
    }
}

impl FromStr for Step {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let turn_str = chars.next().ok_or("No direction at start".to_owned())?;
        let turn = Turn::from_str(&turn_str.to_string())?;
        let distance_str = chars.as_str();
        let distance =
            i32::from_str(distance_str).map_err(|_| format!("Invalid distance: {}", distance_str))?;
        Ok(Step {
            turn: turn,
            distance: distance,
        })
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct Position(pub i32, pub i32);

impl Position {
    pub fn walk(&mut self, direction: &Direction, distance: i32) {
        let vector = direction.to_position_representation();
        self.0 += vector.0 * distance as i32;
        self.1 += vector.1 * distance as i32;
    }

    pub fn distance_from_origo(&self) -> u32 {
        (self.0.abs() + self.1.abs()) as u32
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn(&self, turn: &Turn) -> Direction {
        match *turn {
            Turn::Right => self.turn_right(),
            Turn::Left => self.turn_left(),
        }
    }

    pub fn to_position_representation(&self) -> Position {
        match *self {
            Direction::North => Position(0, 1),
            Direction::East => Position(1, 0),
            Direction::South => Position(0, -1),
            Direction::West => Position(-1, 0),
        }
    }

    fn turn_right(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let first_char = chars.next().ok_or("No direction at start".to_owned())?;
        match first_char {
            'N'|'U' => Ok(Direction::North),
            'E'|'R' => Ok(Direction::East),
            'S'|'D' => Ok(Direction::South),
            'W'|'L' => Ok(Direction::West),
            _ => Err(format!("Invalid direction: {}", first_char)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::{Turn, Step, Direction, Position};

    #[test]
    fn step_from_str_r() {
        let step = Step::from_str("R1").unwrap();
        assert_eq!(Turn::Right, step.turn);
        assert_eq!(1, step.distance);
    }

    #[test]
    fn step_from_str_l() {
        let step = Step::from_str("L99").unwrap();
        assert_eq!(Turn::Left, step.turn);
        assert_eq!(99, step.distance);
    }

    #[test]
    fn step_from_str_negative() {
        let step = Step::from_str("L-15").unwrap();
        assert_eq!(Turn::Left, step.turn);
        assert_eq!(-15, step.distance);
    }

    #[test]
    fn step_from_str_invalid() {
        assert!(Step::from_str("P87").is_err());
    }

    #[test]
    fn position_walk_zero() {
        let mut position = Position(8, -3);
        position.walk(&Direction::North, 0);
        assert_eq!(Position(8, -3), position);
    }

    #[test]
    fn position_walk_south() {
        let mut position = Position(99, 14);
        position.walk(&Direction::South, 15);
        assert_eq!(Position(99, -1), position);
    }

    #[test]
    fn direction_from_str_north() {
        let direction = Direction::from_str("N").unwrap();
        let direction2 = Direction::from_str("U").unwrap();
        assert_eq!(Direction::North, direction);
        assert_eq!(Direction::North, direction2);
    }

    #[test]
    fn direction_from_str_south() {
        let direction = Direction::from_str("S").unwrap();
        let direction2 = Direction::from_str("D").unwrap();
        assert_eq!(Direction::South, direction);
        assert_eq!(Direction::South, direction2);
    }

    #[test]
    fn direction_from_str_west() {
        let direction = Direction::from_str("W").unwrap();
        let direction2 = Direction::from_str("L").unwrap();
        assert_eq!(Direction::West, direction);
        assert_eq!(Direction::West, direction2);
    }

    #[test]
    fn direction_from_str_east() {
        let direction = Direction::from_str("E").unwrap();
        let direction2 = Direction::from_str("R").unwrap();
        assert_eq!(Direction::East, direction);
        assert_eq!(Direction::East, direction2);
    }
}
