extern crate base;

use base::{Part, ProblemSolver};

use std::str::FromStr;

pub fn get_solver() -> Box<ProblemSolver> {
    Box::new(Day2)
}

struct Day2;

impl ProblemSolver for Day2 {
    fn solve(&self, part: Part, input: String) -> Result<String, String> {
        let triangles = match part {
            Part::One => parse_triangle_rows(input)?,
            Part::Two => parse_triangle_column(input)?,
        };
        let num_valid = triangles.iter().filter(|t| t.is_valid()).count();
        Ok(format!("{}", num_valid))
    }
}

fn parse_triangle_rows(input: String) -> Result<Vec<Triangle>, String> {
    let mut triangles = vec![];
    for line in input.lines().map(|line| line.split_whitespace()) {
        let mut sides = vec![];
        for side in line {
            sides.push(u32::from_str(side).map_err(|e| e.to_string())?);
        }
        triangles.push(Triangle::new(sides));
    }
    Ok(triangles)
}

fn parse_triangle_column(input: String) -> Result<Vec<Triangle>, String> {
    let lines: Vec<String> = input.lines().map(|s| s.to_owned()).collect();
    if lines.len() % 3 != 0 {
        return Err("Number of lines must be divisible by 3".to_owned());
    }
    let mut triangles = vec![];
    for three_lines in lines.chunks(3) {
        let chunk_numbers: Vec<Vec<&str>> = three_lines.iter()
            .map(|line| line.split_whitespace().collect())
            .collect();
        if chunk_numbers[0].len() != chunk_numbers[1].len() ||
           chunk_numbers[0].len() != chunk_numbers[2].len() {
            return Err("Lines must be of equal length".to_owned());
        }
        for i in 0..chunk_numbers[0].len() {
            let sides = vec![u32::from_str(chunk_numbers[0][i]).unwrap(),
                             u32::from_str(chunk_numbers[1][i]).unwrap(),
                             u32::from_str(chunk_numbers[2][i]).unwrap()];
            triangles.push(Triangle::new(sides));
        }
    }
    Ok(triangles)
}

#[derive(Debug, Eq, PartialEq)]
pub struct Triangle {
    sides: Vec<u32>,
}

impl Triangle {
    pub fn new(sides: Vec<u32>) -> Self {
        Triangle { sides: sides }
    }

    pub fn is_valid(&self) -> bool {
        let s = &self.sides;
        s.len() == 3 && s[0] + s[1] > s[2] && s[0] + s[2] > s[1] && s[1] + s[2] > s[0]
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
