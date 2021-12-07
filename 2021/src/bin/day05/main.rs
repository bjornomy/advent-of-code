use advent_of_code_2021::puzzle::{self, Puzzle};

use std::collections::HashMap;

struct Part1;
struct Part2;

#[derive(Default, Debug)]
struct Line {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

impl Line {
    fn from_input(line: &str) -> Self {
        let (start, end) = puzzle::split_to_tuple(line, " -> ");
        let (start_x, start_y) = puzzle::split_to_tuple(start, ",");
        let (end_x, end_y) = puzzle::split_to_tuple(end, ",");

        Line {
            start_x: start_x.parse().unwrap(),
            start_y: start_y.parse().unwrap(),
            end_x: end_x.parse().unwrap(),
            end_y: end_y.parse().unwrap(),
        }
    }

    fn is_horizontal(&self) -> bool {
        self.start_y == self.end_y
    }
    fn is_vertical(&self) -> bool {
        self.start_x == self.end_x
    }

    fn covers(&self) -> Vec<String> {
        let mut covers = Vec::new();

        let range;
        if self.is_horizontal() {
            if self.start_x < self.end_x {
                range = self.start_x..(self.end_x + 1);
            } else {
                range = self.end_x..(self.start_x + 1);
            }

            for x in range {
                covers.push(format!("{},{}", x, self.end_y));
            }
        } else {
            if self.start_y < self.end_y {
                range = self.start_y..(self.end_y + 1);
            } else {
                range = self.end_y..(self.start_y + 1);
            }
            for y in range {
                covers.push(format!("{},{}", self.end_x, y));
            }
        }

        covers
    }
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        let count = input
            .split('\n')
            .map(|l| Line::from_input(l))
            .filter(|l| l.is_horizontal() || l.is_vertical())
            .flat_map(|l| l.covers())
            // .inspect(|l| println!("{:?}", l))
            .fold(HashMap::new(), |mut acc, pc| {
                let counter = acc.entry(pc).or_insert(0);
                *counter += 1;
                acc
            })
            .values()
            .filter(|num| **num > 1)
            .count();

        println!("{}", count)
    }
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        println!("NOT STARTED")
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
