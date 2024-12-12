use crate::{Solution, SolutionPair};

pub fn solve() -> SolutionPair {


    let contents = include_str!("../../input/day07_input.txt");

    let solution1 = calibrate(&contents);
    let solution2 = calibrate_2(&contents);

    (Solution::from(solution1), Solution::from(solution2))
}

fn calibrate(data: &str) -> usize {

    let calibration_tests: Vec<Vec<usize>> = 
        data.lines()
        .map(|line| parse_line(line))
        .collect();

    let result: usize = calibration_tests.iter()
        .map(|line| calibrate_line(line))
        .sum();

    result
}

fn calibrate_line(data: &Vec<usize>) -> usize {

    if data.len() < 2 {
        return 0;
    }

    let result = data[0];

    if data.len() == 2 {
        if data[0] == data[1] {
            return data[0];
        }
        else {
            return 0;
        }
    }

    let mut operators = vec![0; data.len() - 2];

    loop {
        let mut operator_counter = 1;
        let mut data_counter = 3;

        let mut product = 0;

        // init product
        if operators[0] == 0 {
            product += data[1] + data[2];
        }
        else {
            product += data[1] * data[2];
        }

        // loop the remaining operators
        while operator_counter < operators.len() {

            if operators[operator_counter] == 0 {
                product += data[data_counter];
            }
            else {
                product *= data[data_counter];
            }

            operator_counter += 1;
            data_counter += 1;
        }

        if product == result {
            return result;
        }

        if operators.iter().all(|&item| item == 1) {
            break;
        }

        let mut operator_idx = 0;
        loop {
            if operators[operator_idx] == 0 {
                operators[operator_idx] = 1;
                break;
            }
            else {
                operators[operator_idx] = 0;
                operator_idx += 1;
            }
        }
    }

    0
}


fn calibrate_2(data: &str) -> usize {

    let calibration_tests: Vec<Vec<usize>> = 
        data.lines()
        .map(|line| parse_line(line))
        .collect();

    let result: usize = calibration_tests.iter()
        .map(|line| calibrate_line_2(line))
        .sum();

    result
}

fn calibrate_line_2(data: &Vec<usize>) -> usize {

    if data.len() < 2 {
        return 0;
    }

    let result = data[0];

    if data.len() == 2 {
        if data[0] == data[1] {
            return data[0];
        }
        else {
            return 0;
        }
    }

    let mut operators = vec![0; data.len() - 2];

    loop {
        let mut operator_counter = 1;
        let mut data_counter = 3;

        let mut product = 0;

        // init product
        if operators[0] == 0 {
            product += data[1] + data[2];
        } 
        else if operators[0] == 1 {
            product += data[1] * data[2];
        }
        else {
            let concat = format!("{}{}", data[1].to_string(), data[2].to_string());
            product += concat.parse::<usize>().expect("Failed to parse String");
        }

        // loop the remaining operators
        while operator_counter < operators.len() {

            let prev_value = product;
            if operators[operator_counter] == 0 {
                product += data[data_counter];
            }
            else if operators[operator_counter] == 1 {
                product *= data[data_counter];
            }
            else {
                let concat = format!("{}{}", product.to_string(), data[data_counter].to_string());
                product = concat.parse::<usize>().expect("Failed to parse String");
            }

            operator_counter += 1;
            data_counter += 1;
        }

        if product == result {
            return result;
        }

        if operators.iter().all(|&item| item == 2) {
            break;
        }

        let mut operator_idx = 0;
        loop {
            if operators[operator_idx] == 0 {
                operators[operator_idx] = 1;
                break;
            }
            else if operators[operator_idx] == 1 {
                operators[operator_idx] = 2;
                break;
            }
            else {
                operators[operator_idx] = 0;
                operator_idx += 1;
            }
        }
    }

    0
}

fn parse_line(data: &str) -> Vec<usize> {

    let mut segments = data.split(" ");

    let mut result = Vec::new();
    while let Some(segment) = segments.next() {
        let segment = segment.replace(":", "");
        result.push(segment.parse::<usize>().expect("Failed to parse line"));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_test() {
        let example = "1234: 1 12 13";

        let result = parse_line(&example);

        assert_eq!(result, vec![1234, 1, 12, 13]);
    }

    #[test]
    fn calibrate_line_test() {
        let good_example = parse_line("3267: 81 40 27");
        let good_example2 = parse_line("9282: 27 45 19 102");
        let bad_example = parse_line("1234: 1 12 13");

        let good_result = calibrate_line(&good_example);
        let good_result2 = calibrate_line(&good_example2);
        let bad_result = calibrate_line(&bad_example);

        assert_eq!(good_result, 3267);
        assert_eq!(good_result2, 9282);
        assert_eq!(bad_result, 0);
    }

    #[test]
    fn example() {
        let example = include_str!("../../samples/day07_sample.txt");

        let result = calibrate(&example);
        let result2 = calibrate_2(&example);

        assert_eq!(result, 3749);
        assert_eq!(result2, 11387);
    }
}