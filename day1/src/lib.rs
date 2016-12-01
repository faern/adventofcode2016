extern crate base;

use base::{Part, ProblemSolver};

use std::collections::HashSet;
use std::str::FromStr;

pub fn get_solver() -> Box<ProblemSolver> {
    Box::new(Day1)
}

struct Day1;

impl ProblemSolver for Day1 {
    fn solve(&self, part: Part, input: String) -> Result<String, String> {
        let steps = parse_input(input)?;
        match part {
            Part::One => Ok(distance_to_endpoint(&steps).to_string()),
            Part::Two => Ok(distance_to_first_path_overlap(&steps)?.to_string()),
        }

    }
}

fn parse_input(input: String) -> Result<Vec<Step>, String> {
    let mut steps = vec![];
    for step_str in input.split(",") {
        let step = Step::from_str(step_str.trim())?;
        steps.push(step);
    }
    Ok(steps)
}

fn distance_to_endpoint(steps: &[Step]) -> u32 {
    let (mut position, mut direction) = start_values();
    for step in steps {
        direction = direction.turn(&step.turn());
        position.walk(&direction, step.distance());
    }
    position.distance_from_origo()
}

fn distance_to_first_path_overlap(steps: &[Step]) -> Result<u32, String> {
    let (mut position, mut direction) = start_values();
    let mut visited = HashSet::new();
    for step in steps {
        direction = direction.turn(&step.turn());
        let normalized_distance = if step.distance() > 0 { 1 } else { -1 };
        for _ in 0..step.distance() {
            position.walk(&direction, normalized_distance);
            if !visited.insert(position) {
                return Ok(position.distance_from_origo());
            }
        }
    }
    Err("The given steps does not cross its own path".to_owned())
}

fn start_values() -> (Position, Direction) {
    (Position(0, 0), Direction::North)
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Turn {
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

struct Step {
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
struct Position(i32, i32);

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

enum Direction {
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::{Turn, Step, Direction, Position, distance_to_endpoint,
                distance_to_first_path_overlap};

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
    fn stand_still() {
        let result = distance_to_endpoint(&[]);
        assert_eq!(0, result);
    }

    #[test]
    fn distance_to_endpoint_single_step() {
        let step = Step::from_str("R1").unwrap();
        let result = distance_to_endpoint(&[step]);
        assert_eq!(1, result);
    }

    #[test]
    fn distance_to_endpoint_two_steps() {
        let steps = [Step::from_str("R100").unwrap(), Step::from_str("R50").unwrap()];
        let result = distance_to_endpoint(&steps);
        assert_eq!(150, result);
    }

    #[test]
    fn distance_to_endpoint_negative() {
        let steps = [Step::from_str("L-40").unwrap(), Step::from_str("R-20").unwrap()];
        let result = distance_to_endpoint(&steps);
        assert_eq!(60, result);
    }

    #[test]
    fn distance_to_endpoint_going_back() {
        let steps = [Step::from_str("R10").unwrap(),
                     Step::from_str("R10").unwrap(),
                     Step::from_str("R10").unwrap()];
        let result = distance_to_endpoint(&steps);
        assert_eq!(10, result);
    }

    #[test]
    fn distance_to_first_path_overlap_no_crossing() {
        let step = Step::from_str("R1").unwrap();
        let result = distance_to_first_path_overlap(&[step]);
        assert!(result.is_err());
    }

    #[test]
    fn distance_to_first_path_overlap_crossing() {
        let steps = [Step::from_str("R8").unwrap(),
                     Step::from_str("R4").unwrap(),
                     Step::from_str("R4").unwrap(),
                     Step::from_str("R8").unwrap()];
        let result = distance_to_first_path_overlap(&steps).unwrap();
        assert_eq!(4, result);
    }
}
