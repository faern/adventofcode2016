extern crate base;

use base::{Part, ProblemSolver};
use base::geo::{Step, Direction, Position};

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

#[cfg(test)]
mod tests {
    use base::geo::Step;
    use std::str::FromStr;
    use super::{distance_to_endpoint, distance_to_first_path_overlap};

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
