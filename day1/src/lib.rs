use std::str::FromStr;

pub fn solve(input: String) -> Result<String, String> {
    let steps = parse_input(input)?;
    Ok(solve_parsed(&steps).to_string())
}

fn parse_input(input: String) -> Result<Vec<Step>, String> {
    let mut steps = vec![];
    for step_str in input.split(",") {
        let step = Step::from_str(step_str.trim())?;
        steps.push(step);
    }
    Ok(steps)
}

fn solve_parsed(steps: &[Step]) -> u32 {
    let mut position = Position(0, 0);
    let mut direction = Direction::North;
    for step in steps {
        direction = direction.turn(&step.turn());
        position.walk(&direction, step.distance());
    }
    (position.0.abs() + position.1.abs()) as u32
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
        let distance = i32::from_str(distance_str).map_err(|_| format!("Invalid distance: {}", distance_str))?;
        Ok(Step {
            turn: turn,
            distance: distance,
        })
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Position(i32, i32);

impl Position {
    pub fn walk(&mut self, direction: &Direction, distance: i32) {
        let vector = direction.to_position_representation();
        self.0 += vector.0 * distance as i32;
        self.1 += vector.1 * distance as i32;
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
    use super::{Turn, Step, Direction, Position, solve_parsed};
    use std::str::FromStr;

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
        let result = solve_parsed(&[]);
        assert_eq!(0, result);
    }

    #[test]
    fn solve_parsed_single_step() {
        let step = Step::from_str("R1").unwrap();
        let result = solve_parsed(&[step]);
        assert_eq!(1, result);
    }

    #[test]
    fn solve_parsed_two_steps() {
        let steps = [
            Step::from_str("R100").unwrap(),
            Step::from_str("R50").unwrap(),
        ];
        let result = solve_parsed(&steps);
        assert_eq!(150, result);
    }

    #[test]
    fn solve_parsed_negative() {
        let steps = [
            Step::from_str("L-40").unwrap(),
            Step::from_str("R-20").unwrap(),
        ];
        let result = solve_parsed(&steps);
        assert_eq!(60, result);
    }

    #[test]
    fn solve_parsed_going_back() {
        let steps = [
            Step::from_str("R10").unwrap(),
            Step::from_str("R10").unwrap(),
            Step::from_str("R10").unwrap(),
        ];
        let result = solve_parsed(&steps);
        assert_eq!(10, result);
    }
}
