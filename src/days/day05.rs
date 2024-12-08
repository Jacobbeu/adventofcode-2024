use crate::{Solution, SolutionPair};
use std::collections::HashMap;

pub fn solve() -> SolutionPair {

    let contents = include_str!("../../input/day05_input.txt");

    let solution1 = parse_safety_manual(&contents);
    let solution2 = parse_incorrect_safety_manual(&contents);

    (Solution::from(solution1), Solution::from(solution2))
}

fn parse_safety_manual(data: &str) -> usize {

    let mut safety_pages: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut lines = data.lines();

    while let Some(line) = lines.next() {
        if line.eq("") {
            break;
        }
        let mut splits = line.split("|");
        let page_number = splits.next().unwrap().parse::<usize>().expect("Must be numeric");
        let preceded_by = splits.next().unwrap().parse::<usize>().expect("Must be numeric");

        match safety_pages.get_mut(&page_number) {
            Some(pages) => { pages.push(preceded_by); },
            None => { safety_pages.insert(page_number, vec![preceded_by]); },
        }
    }

    let mut total: usize = 0;
    while let Some(line) = lines.next() { 
        let reports: Vec<usize> = line.split(",").map(|s| s.parse::<usize>().expect("report must be numeric")).collect();
        if let Some(value) = middle_page_value(&reports, &safety_pages) {
            total += value;
        }
    }

    total
}

fn parse_incorrect_safety_manual(data: &str) -> usize {

    let mut safety_pages: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut safety_pages_str = String::new();
    let mut lines = data.lines();

    while let Some(line) = lines.next() {
        if line.eq("") {
            break;
        }
        let mut splits = line.split("|");
        let page_number = splits.next().unwrap().parse::<usize>().expect("Must be numeric");
        let preceded_by = splits.next().unwrap().parse::<usize>().expect("Must be numeric");

        match safety_pages.get_mut(&page_number) {
            Some(pages) => { pages.push(preceded_by); },
            None => { safety_pages.insert(page_number, vec![preceded_by]); },
        }
        safety_pages_str.push_str(line);
        safety_pages_str.push_str("\r\n");
    }

    let pairs: Vec<(usize, usize)> = safety_pages_str.lines()
        .map(|l| {
            let mut s = l.split("|");
            (s.next().unwrap().parse::<usize>().expect("must be numeric"), s.next().unwrap().parse::<usize>().expect("must be numeric"))
        })
        .collect();

    // let rankings = build_ranking(&safety_pages_str);

    let mut total: usize = 0;
    while let Some(line) = lines.next() { 
        let reports: Vec<usize> = line.split(",").map(|s| s.parse::<usize>().expect("report must be numeric")).collect();
        if middle_page_value(&reports, &safety_pages).is_none() {
            total += corrected_middle_page_value(&reports, &pairs).unwrap_or(0);
        }
    }

    total
}

fn middle_page_value(data: &Vec<usize>, pages: &HashMap<usize, Vec<usize>>) -> Option<usize> {

    let mut counter = 0;
    for record in data.iter() {
        let record_slice = data.iter().skip(counter + 1);
        let Some(map) = pages.get(record) else {

            if (counter + 1) == data.len() {
                break;
            }
            else {
                return None;
            }
        };

        for slice in record_slice {
            if !map.contains(slice) {
                return None;
            }
        }
        counter += 1;
    }
    
    Some(data.get(data.len() / 2).unwrap().clone())
}

fn corrected_middle_page_value(data: &Vec<usize>, rankings: &Vec<(usize, usize)>) -> Option<usize> {

    let mut report = data.clone();

    report.sort_by(|entry1, entry2| {
        let ranking = rankings.iter().find(|rank| {
            rank.0 == *entry1 && rank.1 == *entry2 || rank.0 == *entry2 && rank.1 == *entry1
        });

        if let Some(ranking) = ranking {
            if ranking.0 == *entry1 {
                std::cmp::Ordering::Less
            } 
            else {
                std::cmp::Ordering::Greater
            }
        }
        else if rankings.iter().any(|rank| rank.0 == *entry1 || rank.1 == *entry2) {
            std::cmp::Ordering::Less
        }
        else {
            std::cmp::Ordering::Greater
        }
    });

    let middle = report.get(report.len()/ 2).unwrap();
    Some(*middle)
}

// fn build_ranking(data: &str) -> Vec<usize> {

//     println!("build rankings data: {data}");

//     let mut pairs: Vec<(usize, usize)> = data.lines()
//         .map(|l| {
//             let mut s = l.split("|");
//             (s.next().unwrap().parse::<usize>().expect("must be numeric"), s.next().unwrap().parse::<usize>().expect("must be numeric"))
//         })
//         .collect();

//     let numbers = pairs.iter()
//         .fold(Vec::new(), | mut output, tuple | {
//             if !output.contains(&tuple.0) {
//                 output.push(tuple.0);
//             }
//             if !output.contains(&tuple.1) {
//                 output.push(tuple.1);
//             }
//             output
//         });

//     let mut ranking: Vec<usize> = Vec::new();

//     println!("numbers: {:?}", numbers);

//     while pairs.len() > 0 {
//         println!("pairs: {:?}", pairs);
//         let pages = pairs.iter()
//         .fold(Vec::new(), | mut output, tuple | {
//             if !output.contains(&tuple.0) {
//                 output.push(tuple.0);
//             }
//             output
//         });
//         let highest_number = numbers.iter().find(|page| if !pages.contains(page) { !ranking.contains(page) } else { false }).unwrap();
//         ranking.insert(0, *highest_number);
//         pairs.retain(|tuple| tuple.1 != *highest_number);
//     }

//     let missing_number = numbers.iter().find(|num| ranking.contains(num) == false).unwrap();
//     ranking.insert(0, *missing_number);

//     ranking
// }

#[cfg(test)]
mod tests {
    use super::*;

//     #[test]
//     fn example_build_ranking() {
//         let example = 
// "47|53
// 97|13
// 97|61
// 97|47
// 75|29
// 61|13
// 75|53
// 29|13
// 97|29
// 53|29
// 61|53
// 97|53
// 61|29
// 47|13
// 75|47
// 97|75
// 47|61
// 75|61
// 47|29
// 75|13
// 53|13";

//         let result = build_ranking(&example);

//         assert_eq!(result, vec![97, 75, 47, 61, 53, 29, 13])

//     }

    #[test]
    fn example() {
        let example = 
"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let result = parse_safety_manual(&example);
        let result2 = parse_incorrect_safety_manual(&example);

        assert_eq!(result, 143);
        assert_eq!(result2, 123);

    }
}