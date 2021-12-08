use advent_of_code_2021::puzzle::{self, Puzzle};

use std::collections::HashMap;

struct Part1;
struct Part2;

fn move_cost(map: &HashMap<isize, isize>, target: isize) -> isize {
    map.into_iter()
        .map(|(height, crabs_at_height)| {
            let cost = (height - target).abs();
            cost * crabs_at_height
        })
        .sum()
}

fn expensive_move_cost(map: &HashMap<isize, isize>, target: isize) -> isize {
    map.into_iter()
        .map(|(height, crabs_at_height)| {
            let diff = (height - target).abs();
            let mut cost = diff;
            for i in 0..diff {
                cost += i;
            }
            cost * crabs_at_height
        })
        .sum()
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        // array to track num crabs at height
        let crabs: HashMap<isize, isize> =
            input
                .split(',')
                .map(|n| n.parse().unwrap())
                .fold(HashMap::new(), |mut acc, height| {
                    let counter = acc.entry(height).or_insert(0);
                    *counter += 1;
                    acc
                });

        let min = *crabs.keys().min().unwrap_or(&0isize);
        let max = *crabs.keys().max().unwrap_or(&0isize);

        let mut lowest_cost = isize::MAX;
        for target_height in min..max {
            let cur_cost = move_cost(&crabs, target_height);
            if cur_cost < lowest_cost {
                lowest_cost = cur_cost
            }
        }

        println!("{}", lowest_cost)
    }
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        let crabs: HashMap<isize, isize> =
            input
                .split(',')
                .map(|n| n.parse().unwrap())
                .fold(HashMap::new(), |mut acc, height| {
                    let counter = acc.entry(height).or_insert(0);
                    *counter += 1;
                    acc
                });

        let min = *crabs.keys().min().unwrap_or(&0isize);
        let max = *crabs.keys().max().unwrap_or(&0isize);

        let mut lowest_cost = isize::MAX;
        for target_height in min..max {
            let cur_cost = expensive_move_cost(&crabs, target_height);
            if cur_cost < lowest_cost {
                lowest_cost = cur_cost
            }
        }

        println!("{}", lowest_cost)
    }
}

const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("example.txt");

fn main() {
    print!("Part 1: ");
    Part1::solve(INPUT);

    print!("Part 2: ");
    Part2::solve(INPUT);
}
