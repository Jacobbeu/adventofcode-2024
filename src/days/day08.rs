use std::cmp::Ordering;

use crate::{Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    let contents = include_str!("../../input/day08_input.txt");

    let solution1 = get_unique_antinodes(&contents);
    let solution2 = 0;

    (Solution::from(solution1), Solution::from(solution2))
}

struct Map {
    width: usize,
    height: usize,
    points: Vec<Point>,
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    designation: char,
}

fn get_unique_antinodes(data: &str) -> usize {

    let map = parse_map(data);

    let mut unique_antennas_designations = map.points.iter().map(|point| point.designation).collect::<Vec<char>>();
    unique_antennas_designations.sort();
    unique_antennas_designations.dedup();

    let mut all_antinodes = Vec::new();

    for antenna_designation in unique_antennas_designations {
        let antennas: Vec<_> = 
            map.points.iter()
            .filter(|point| point.designation == antenna_designation)
            .collect();

        let mut antinodes: Vec<Point> = 
            antennas.iter()
                .fold(Vec::new(), | mut output, antenna| {
                    let mut points = 
                        antennas.iter()
                            .fold(Vec::new(), | mut inner_output, another_antenna| {
                                if antenna != another_antenna {
                                    let distance_x = another_antenna.x - antenna.x;
                                    let distance_y = another_antenna.y - antenna.y;

                                    let x = antenna.x - distance_x;
                                    let y = antenna.y - distance_y;

                                    if (0..map.width as i32).contains(&x) && (0..map.height as i32).contains(&y) {
                                        inner_output.push(Point { x, y, designation: '#'});
                                    }
                                }

                                inner_output
                            });
                    output.append(&mut points);
                    output
                });

        all_antinodes.append(&mut antinodes);
    }

    all_antinodes.sort_by(|a, b| {
        if a.x < b.x {
            Ordering::Less
        }
        else if a.x > b.x {
            Ordering::Greater
        }
        else if a.y < b.y {
            Ordering::Less
        }
        else if a.y > b.y {
            Ordering::Greater
        }
        else {
            Ordering::Equal
        }
    });
    all_antinodes.dedup();
    all_antinodes.len()
}

fn parse_map(data: &str) -> Map {

    let width = data.lines().next().unwrap().len();
    let height = data.lines().count();

    let points: Vec<Point> = 
        data.lines()
        .enumerate()
        .fold(Vec::new(), |mut output, (y, line)| {
            let mut test = 
                line.chars()
                .enumerate()
                .fold(Vec::new(), | mut output, (x, letter)| {
                    if letter != '.' {
                        output.push(Point { x: x as i32, y: y as i32, designation: letter });
                    }

                    output
                });

            output.append(&mut test);
            output
        });

    Map { width, height, points }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_map_test() {
        let example = include_str!("../../samples/day08_sample.txt");

        let result = parse_map(example);

        assert_eq!(result.width, 12);
        assert_eq!(result.height, 12);
        assert_eq!(result.points.len(), 7);

        assert_eq!(result.points[0], Point { x: 8, y: 1, designation: '0' });
        assert_eq!(result.points[1], Point { x: 5, y: 2, designation: '0' });
        assert_eq!(result.points[2], Point { x: 7, y: 3, designation: '0' });
        assert_eq!(result.points[3], Point { x: 4, y: 4, designation: '0' });
        assert_eq!(result.points[4], Point { x: 6, y: 5, designation: 'A' });
        assert_eq!(result.points[5], Point { x: 8, y: 8, designation: 'A' });
        assert_eq!(result.points[6], Point { x: 9, y: 9, designation: 'A' });
    }

    #[test]
    fn example() {
        let example = include_str!("../../samples/day08_sample.txt");

        let result = get_unique_antinodes(&example);

        assert_eq!(result, 14);
    }
}