use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::error::Error;

pub fn solve() -> SolutionPair {

    let contents = read_to_string("input/day02_input.txt").expect("failed to parse input file");

    let mut lines = contents.lines();

    let mut solution1 = 0;
    let mut solution2 = 0;

    while let Some(line) = lines.next() {
        if parse_report(line).expect("Failed to parse report.") {
            solution1 += 1;
            solution2 += 1;
            continue;
        }

        if dampen_report(line).expect("Failed to parse dampened report.") {
            solution2 += 1;
        }
    }

    (Solution::from(solution1), Solution::from(solution2))
}

fn parse_report(data: &str) -> Result<bool, Box<dyn Error>> {

    let mut report_entries = data.split_whitespace();
    let mut entries = Vec::new();

    while let Some(entry) = report_entries.next() {
        let entry = entry.parse::<i32>()?;
        entries.push(entry);
    }

    parse_data(&entries)
}

fn parse_data(data: &Vec<i32>) -> Result<bool, Box<dyn Error>> {

    let mut data_iter = data.iter();

    let first_entry = data_iter.next().unwrap();
    let mut last_entry = data_iter.next().unwrap();
    let change = last_entry - first_entry;

    if change == 0 || change.abs() > 3 {
        return Ok(false);
    }
    
    while let Some(entry) = data_iter.next() {
        let difference = entry - last_entry;

        if difference == 0 || difference.abs() > 3 {
            return Ok(false);
        }

        last_entry = entry;

        if change < 0 && difference > 0 {
            return Ok(false);
        }

        if change > 0 && difference < 0 {
            return Ok(false);
        }
    }

    Ok(true)
}

fn dampen_report(data: &str) -> Result<bool, Box<dyn Error>> {

    let mut report_entries = data.split_whitespace();
    let mut entries = Vec::new();

    while let Some(entry) = report_entries.next() {
        let entry = entry.parse::<i32>()?;
        entries.push(entry);
    }

    for idx in 0..entries.len() {
        let mut modified_entries = entries.clone();
        modified_entries.remove(idx);

        if parse_data(&modified_entries)? {
            return Ok(true);
        }
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = 
"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let results = vec![true, false, false, false, false, true];
        let mut lines = example.lines();

        assert_eq!(parse_report(lines.next().unwrap()).expect("Parse Failed"), results[0]);
        assert_eq!(parse_report(lines.next().unwrap()).expect("Parse Failed"), results[1]);
        assert_eq!(parse_report(lines.next().unwrap()).expect("Parse Failed"), results[2]);
        assert_eq!(parse_report(lines.next().unwrap()).expect("Parse Failed"), results[3]);
        assert_eq!(parse_report(lines.next().unwrap()).expect("Parse Failed"), results[4]);
        assert_eq!(parse_report(lines.next().unwrap()).expect("Parse Failed"), results[5]);
    }

    #[test]
    fn example_2() {
        let example = 
"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let results = vec![true, false, false, true, true, true];
        let mut lines = example.lines();

        assert_eq!(parse_report(lines.next().unwrap()).expect("Parse Failed"), results[0]);
        assert_eq!(dampen_report(lines.next().unwrap()).expect("Parse Failed"), results[1]);
        assert_eq!(dampen_report(lines.next().unwrap()).expect("Parse Failed"), results[2]);
        assert_eq!(dampen_report(lines.next().unwrap()).expect("Parse Failed"), results[3]);
        assert_eq!(dampen_report(lines.next().unwrap()).expect("Parse Failed"), results[4]);
        assert_eq!(parse_report(lines.next().unwrap()).expect("Parse Failed"), results[5]);
    }
}