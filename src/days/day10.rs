use crate::{Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let contents = include_str!("../../input/day10_input.txt");

    let result = calulate_trailhead_score(&contents);

    (Solution::from(result.0), Solution::from(result.1))
}

struct Map {
    width: usize,
    height: usize,
    points: Vec<Point>,
}

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    height: usize,
}

fn parse_map(data: &str) -> Map {

    let width = data.lines().next().unwrap().len();
    let height = data.lines().count();

    let points: Vec<Point> = 
        data.lines()
        .enumerate()
        .fold(Vec::new(), |mut output, (y, line)| {
            let mut row_of_points: Vec<Point> = 
                line.chars()
                .enumerate()
                .map(|(x, letter)| {
                    let height = letter.to_string().parse::<usize>().expect("NaN found");
                    Point { x, y, height }
                })
                .collect();

            output.append(&mut row_of_points);
            output
        });

    Map { width, height, points }
}

fn calulate_trailhead_score(data: &str) -> (usize, usize) {
    
    let topo_map = parse_map(data);
    let trailheads: Vec<&Point> =
        topo_map.points.iter()
        .filter(|point| point.height == 0)
        .collect();

    let result: Vec::<(usize, usize)> = 
        trailheads
        .iter()
        .map(|point| {
            let ret_val = walk_paths(&topo_map, point, &mut Vec::new());
            (ret_val.0.iter().count(), ret_val.1)
        })
        .collect();

    (result.iter().map(|r|r.0).sum(), result.iter().map(|r|r.1).sum())
}

fn walk_paths<'a>(topo_map: &'a Map, starting_point: &Point, acc: &mut Vec<&'a Point>) -> (Vec<&'a Point>, usize) {

    let mut total = 0;
    // look north
    if starting_point.y != 0 {
        if let Some(north_point) = topo_map.points.iter().find(|point| point.x == starting_point.x && point.y == starting_point.y - 1 && point.height == starting_point.height + 1) {
            if north_point.height == 9 {
                if !acc.contains(&north_point) {
                    acc.push(north_point);
                }
                total += 1;
            }
            else {
                let ret_val = walk_paths(topo_map, north_point, acc);
                *acc = ret_val.0;
                total += ret_val.1;
            }
        }
    }

    // look south
    if let Some(south_point) = topo_map.points.iter().find(|point| point.x == starting_point.x && point.y == starting_point.y + 1 && point.height == starting_point.height + 1) {
        if south_point.height == 9 {
            if !acc.contains(&south_point) {
                acc.push(south_point);
            }
            total += 1;
        }
        else {
            let ret_val = walk_paths(topo_map, south_point, acc);
            *acc = ret_val.0;
            total += ret_val.1;
        }
    }

    // look west
    if starting_point.x != 0 {
        if let Some(west_point) = topo_map.points.iter().find(|point| point.x == starting_point.x - 1 && point.y == starting_point.y && point.height == starting_point.height + 1) {
            if west_point.height == 9 {
                if !acc.contains(&west_point) {
                    acc.push(west_point);
                }
                total += 1;
            }
            else {
                let ret_val = walk_paths(topo_map, west_point, acc);
                *acc = ret_val.0;
                total += ret_val.1;
            }
        }
    }

    // look east
    if let Some(east_point) = topo_map.points.iter().find(|point| point.x == starting_point.x + 1 && point.y == starting_point.y && point.height == starting_point.height + 1) {
        if east_point.height == 9 {
            if !acc.contains(&east_point) {
                acc.push(east_point);
            }
            total += 1;
        }
        else {
            let ret_val = walk_paths(topo_map, east_point, acc);
            *acc = ret_val.0;
            total += ret_val.1;
        }
    }

    (acc.to_vec(), total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_example() {
        let example = include_str!("../../samples/day10_sample.txt");

        let result = calulate_trailhead_score(&example);

        assert_eq!(result.0, 36);
        assert_eq!(result.1, 81);
    }
}