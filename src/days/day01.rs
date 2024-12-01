use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::error::Error;
use std::collections::HashMap;

pub fn solve() -> SolutionPair {

    let contents = read_to_string("input/day01_input.txt").expect("failed to parse input file");

    let distance = total_distance(&contents).expect("Failed to parse contents");
    let similarity = total_similarity(&contents).expect("Failed to parse contents");

    let solution1 = distance;
    let solution2 = similarity;

    (Solution::from(solution1), Solution::from(solution2))
}

fn total_distance(data: &str) -> Result<u32, Box<dyn Error>> {
    let result = parse_file(data)?;
    let list_one = result.0;
    let list_two = result.1;

    let mut total_distance: u32 = 0;

    for i in 0..list_one.len() {
        let distance = (list_one[i] - list_two[i]).abs() as u32;
        total_distance += distance;
    }

    Ok(total_distance)
}

fn total_similarity(data: &str) -> Result<u32, Box<dyn Error>> {

    let result = parse_file(data)?;
    let list_one = result.0;
    let list_two = result.1;

    let mut list_two_map = HashMap::new();

    for int in list_two.iter() {
        let count = list_two_map.entry(int).or_insert(0);
        *count += 1;
    }

    let mut total_similarity = 0;
    
    for int in list_one.iter() {
        let count = list_two_map.get(int).unwrap_or(&0);
        total_similarity += int * count;
    }

    Ok(total_similarity.try_into().unwrap())
}

fn parse_file(data: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let mut lines = data.lines();

    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();

    while let Some(line) = lines.next() {

        let mut numbers = line.split_whitespace();

        let number_one = numbers.next().unwrap().parse::<i32>()?;
        let number_two = numbers.next().unwrap().parse::<i32>()?;

        list_one.push(number_one);
        list_two.push(number_two);
    }

    list_one.sort();
    list_two.sort();

    Ok((list_one, list_two))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_example() {

        let example = 
"3   4
4   3
2   5
1   3
3   9
3   3";

        let result :u32 = total_distance(&example).expect("Example data failed to parse");

        assert_eq!(result, 11);
    }
    #[test]
    fn day01_example_2() {

        let example = 
"3   4
4   3
2   5
1   3
3   9
3   3";

        let result :u32 = total_similarity(&example).expect("Example data failed to parse");

        assert_eq!(result, 31);
    }
}