use crate::{Solution, SolutionPair};
use std::collections::HashMap;

pub fn solve() -> SolutionPair {
    let contents = include_str!("../../input/day12_input.txt");

    let solution1 = calculate_lumber_cost(&contents);
    let solution2 = 0;

    (Solution::from(solution1), Solution::from(solution2))
}

#[derive(Debug)]
struct Plot {
    x: i32,
    y: i32,
    plant: char,
}

impl Plot {
    fn get_fencing_needed(&self, plots: &Vec<Plot>) -> usize {
        let mut fencing = 0;

        // up
        if plots.iter().find(|plot| plot.x == self.x && plot.y == (self.y - 1) && plot.plant == self.plant).is_none() {
            fencing += 1;
        }

        // down
        if plots.iter().find(|plot| plot.x == self.x && plot.y == (self.y + 1) && plot.plant == self.plant).is_none() {
            fencing += 1;
        }

        // left
        if plots.iter().find(|plot| plot.x == (self.x - 1) && plot.y == self.y && plot.plant == self.plant).is_none() {
            fencing += 1;
        }

        // right
        if plots.iter().find(|plot| plot.x == (self.x + 1) && plot.y == self.y && plot.plant == self.plant).is_none() {
            fencing += 1;
        }

        fencing
    }

    fn is_neighbor(&self, plot: &Plot) -> bool {

        if self.plant != plot.plant {
            false
        }
        else if self.x == plot.x && ((self.y - 1) == plot.y || (self.y + 1) == plot.y) {
            true
        }
        else if self.y == plot.y && ((self.x - 1) == plot.x || (self.x + 1) == plot.x) {
            true
        }
        else {
            false
        }
    }
}

#[derive(Debug)]
struct Region {
    plots: Vec<Plot>,
    fencing_needed: usize,
}

impl Region {
    fn push_neighbors(&mut self, plots: &mut Vec<Plot>) {

        'outer: loop {
            for iter in 0..plots.len() {
                let external_plot = &plots[iter];
                if self.plots.iter().find(|region_plot| region_plot.is_neighbor(external_plot)).is_some()
                {
                    self.plots.push(plots.remove(iter));
                    continue 'outer;
                }
            }
            break;
        }

        for plot in self.plots.iter() {
            self.fencing_needed += plot.get_fencing_needed(&self.plots);
        }
    }
}

fn parse_map(data: &str) -> Vec<Plot> {
    data.lines()
        .enumerate()
        .fold(Vec::new(), |mut output, (y, line)| {
            let mut row_of_points: Vec<Plot> = 
                line.chars()
                .enumerate()
                .map(|(x, plant)| {
                    Plot { x: x as i32, y: y as i32, plant }
                })
                .collect();

            output.append(&mut row_of_points);
            output
        })
}

fn calculate_lumber_cost(data: &str) -> usize {

    let mut plots: Vec<Plot> = parse_map(data);
    let mut regions: Vec<Region> = Vec::new();

    while let Some(plot) = plots.pop() {

        let mut region = Region { plots: Vec::new(), fencing_needed: 0 };

        region.plots.push(plot);
        region.push_neighbors(&mut plots);
        
        regions.push(region);
    }

    regions.iter().map(|region| region.plots.len() * region.fencing_needed).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = include_str!("../../samples/day12_sample.txt");

        let result1 = calculate_lumber_cost(&example);

        assert_eq!(result1, 1930);
    }
}