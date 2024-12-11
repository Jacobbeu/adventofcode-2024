use crate::{Solution, SolutionPair};

pub fn solve() -> SolutionPair {

    let contents = include_str!("../../input/day06_input.txt");

    let solution1 = plan_guard_route(&contents);
    let solution2 = 0;

    (Solution::from(solution1), Solution::from(solution2))
}

struct Guard {
    x: usize,
    y: usize,
    symbol: char,
}

impl Guard {
    fn rotate(&mut self) {
        if self.symbol == '^' {
            self.symbol = '>';
        }
        else if self.symbol == '>' {
            self.symbol = 'V';
        }
        else if self.symbol == 'V' {
            self.symbol = '<';
        }
        else if self.symbol == '<' {
            self.symbol = '^';
        }
    }
}

fn plan_guard_route(data: &str) -> usize {

    let mut route_map: Vec<Vec<char>> = 
        data.lines()
        .map(|line| -> Vec<char> {
            line.chars().collect()
        })
        .collect();

    let mut guard = data.lines()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars()
                .enumerate()
                .find_map(|(x, letter)| {
                    if letter == '^' || letter == '>' || letter == 'V' || letter == '<' {
                        Some(Guard {x, y, symbol: letter})
                    }
                    else {
                        None
                    }
                })
        })
        .unwrap();

    
    let Some(row) = route_map.get_mut(guard.y) else {
        return 0;
    };

    let Some(column) = row.get_mut(guard.x) else {
        return 0;
    };

    *column = 'X';

    loop {
        println!("X: {}, Y: {}, Guard: {}", guard.x, guard.y, guard.symbol);
        let mut next_guard_x = guard.x;
        let mut next_guard_y = guard.y;

        if guard.symbol == '^' {
            next_guard_y -= 1;
        }
        else if guard.symbol == '>' {
            next_guard_x += 1;
        }
        else if guard.symbol == 'V' { 
            next_guard_y += 1;
        }
        else if guard.symbol == '<' {
            next_guard_x -= 1;
        }

        let Some(row) = route_map.get_mut(next_guard_y) else {
            break;
        };

        let Some(column) = row.get_mut(next_guard_x) else {
            break;
        };

        if *column == '#' {
            guard.rotate();
        } else {
            guard.x = next_guard_x;
            guard.y = next_guard_y;
            *column = 'X';
        }
    }

    println!("Map:");
    route_map.iter().for_each(|line| {
        println!("{:?}", line);
    });

    let total: usize = route_map.iter()
        .map(|line| {
            line.iter()
                .filter(|letter| **letter == 'X')
                .count()
        })
        .into_iter()
        .sum();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06_example() {
        let example = include_str!("../../samples/day06_sample.txt");

        let result = plan_guard_route(&example);

        assert_eq!(result, 41);
    }
}