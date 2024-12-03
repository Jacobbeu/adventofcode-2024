use crate::{Solution, SolutionPair};
use std::error::Error;
use regex::Regex;

pub fn solve() -> SolutionPair {

    let contents = include_str!("../../input/day03_input.txt");

    let solution1 = mul_program(contents).expect("Failed to parse contents.");
    let solution2 = mul_program_2(contents).expect("Failed to parse contents.");

    (Solution::from(solution1), Solution::from(solution2))
}

fn mul_program(data: &str) -> Result<usize, Box<dyn Error>> {

    let regex = Regex::new(r"(?m)mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let result = regex.captures_iter(data);

    let mut total = 0;

    for mat in result {
        let operand1 = mat.get(1).map_or("0", |m| m.as_str());
        let operand2 = mat.get(2).map_or("0", |m| m.as_str());

        let operand1 = operand1.parse::<usize>()?;
        let operand2 = operand2.parse::<usize>()?;

        total += operand1 * operand2;
    }

    Ok(total)
}

fn mul_program_2(data: &str) -> Result<usize, Box<dyn Error>> {
    
    let regex = Regex::new(r"(?m)(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();
    let result = regex.captures_iter(data);

    let mut total = 0;
    let mut enabled_flag = true;
    
    for mat in result {
        if mat.get(4).is_some() {
            enabled_flag = true;
            continue;
        }

        if mat.get(5).is_some() {
            enabled_flag = false;
            continue;
        }

        if !enabled_flag {
            continue;
        }

        let operand1 = mat.get(2).map_or("0", |m| m.as_str());
        let operand2 = mat.get(3).map_or("0", |m| m.as_str());

        let operand1 = operand1.parse::<usize>()?;
        let operand2 = operand2.parse::<usize>()?;

        total += operand1 * operand2;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let result = mul_program(&example).expect("Failed to parse example.");

        assert_eq!(result, 161);
    }

    #[test]
    fn example_2() {
        let example = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let result = mul_program_2(&example).expect("Failed to parse example.");

        assert_eq!(result, 48);
    }
}