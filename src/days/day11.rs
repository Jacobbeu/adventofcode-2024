use crate::{Solution, SolutionPair};
use std::collections::HashMap;

pub fn solve() -> SolutionPair {
    let contents = include_str!("../../input/day11_input.txt");

    let solution1 = get_stone_count(&contents, 25);
    let solution2 = get_stone_count(&contents, 75);

    (Solution::from(solution1), Solution::from(solution2))
}

fn get_stone_count(data: &str, blinks: usize) -> usize {

    let mut stones: HashMap<usize, usize> = 
        data.split_whitespace()
        .fold(HashMap::new(), |mut output, stone| {
            let stone = stone.parse::<usize>().expect("NaN");
            match output.get(&stone) {
                Some(count) => output.insert(stone, count + 1),
                None => output.insert(stone, 1)
            };
            output
        });

    for counter in 0..blinks {

        println!("Blink count: {counter}");

        let mut temp_stones: HashMap<usize, usize> = HashMap::new();

        for (stone, qty) in stones.into_iter() {
            let transformation = transform_stone(stone);
            let stone1 = transformation.0;
            let stone2 = transformation.1;

            match temp_stones.get(&stone1) {
                Some(count) => temp_stones.insert(stone1, count + qty),
                None => temp_stones.insert(stone1, qty)
            };

            if let Some(stone2) = stone2 {
                match temp_stones.get(&stone2) {
                    Some(count) => temp_stones.insert(stone2, count + qty),
                    None => temp_stones.insert(stone2, qty)
                };
            }
        }

        stones = temp_stones.clone();
    }

    stones.into_values().sum()
}

fn transform_stone(stone: usize) -> (usize, Option<usize>) {
    if stone == 0 {
        return (1, None);
    }

    let stone_string = stone.to_string();
    if stone_string.len() % 2 == 0 {
        let split_stone = stone_string.split_at(stone_string.len() / 2);
        return (split_stone.0.parse::<usize>().expect("NaN found"), Some(split_stone.1.parse::<usize>().expect("NaN found")));
    }

    (stone * 2024, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tranform_stone_example() {
        let example1 = 0;
        let example2 = 1;
        let example3 = 10;
        let example4 = 1000;
        let example5 = 99;
        let example6 = 999;


        let result1 = transform_stone(example1);
        let result2 = transform_stone(example2);
        let result3 = transform_stone(example3);
        let result4 = transform_stone(example4);
        let result5 = transform_stone(example5);
        let result6 = transform_stone(example6);

        assert_eq!(result1.0, 1);
        assert_eq!(result2.0, 2024);
        assert_eq!(result3.0, 1);
        assert_eq!(result3.1, Some(0));
        assert_eq!(result4.0, 10);
        assert_eq!(result4.1, Some(0));
        assert_eq!(result5.0, 9);
        assert_eq!(result5.1, Some(9));
        assert_eq!(result6.0, 2021976);
    }

    #[test]
    fn example() {
        let example = include_str!("../../samples/day11_sample.txt");

        let result1 = get_stone_count(&example, 6);
        let result2 = get_stone_count(&example, 25);

        assert_eq!(result1, 22);
        assert_eq!(result2, 55312);
    }
}