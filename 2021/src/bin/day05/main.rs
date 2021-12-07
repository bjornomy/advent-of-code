use advent_of_code_2021::puzzle::{self, Puzzle};

use std::collections::HashMap;

struct Part1;
struct Part2;

fn is_diagonal(x1: usize, x2: usize, y1: usize, y2: usize) -> bool {
    let x_diff = (x1 as i32 - x2 as i32).abs();
    let y_diff = (y1 as i32 - y2 as i32).abs();

    x_diff == y_diff
}

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

    fn is_diagonal(&self) -> bool {
        is_diagonal(self.start_x, self.end_x, self.start_y, self.end_y)
    }

    fn covers(&self) -> Vec<String> {
        let mut covers = Vec::new();

        // covers.push(format!(
        //     "{},{} -> {},{}",
        //     self.start_x, self.start_y, self.end_x, self.end_y
        // ));

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

    fn covers_with_diags(&self) -> Vec<String> {
        let mut covers = Vec::new();

        if self.is_horizontal() || self.is_vertical() {
            return self.covers();
        }

        // covers.push(format!(
        //     "{},{} -> {},{}",
        //     self.start_x, self.start_y, self.end_x, self.end_y
        // ));

        // covers.push(format!("{},{}", -1, -1));

        if self.start_x < self.end_x && self.start_y < self.end_y {
            // go up and right --> x1 < x2 & y1 < y2
            for i in 0..(self.end_x - self.start_x + 1) {
                covers.push(format!("{},{}", self.start_x + i, self.start_y + i));
            }
        } else if self.start_x < self.end_x && self.start_y > self.end_y {
            // go up and left  --> x1 < x2 & y1 > y2
            for i in 0..(self.end_x - self.start_x + 1) {
                covers.push(format!("{},{}", self.start_x + i, self.start_y - i));
            }
        } else if self.start_x > self.end_x && self.start_y < self.end_y {
            // go down and right --> x1 > x2 & y1 < y2
            for i in 0..(self.start_x - self.end_x + 1) {
                covers.push(format!("{},{}", self.start_x - i, self.start_y + i));
            }
        } else if self.start_x > self.end_x && self.start_y > self.end_y {
            // go down and left  --> x1 > x2 & y1 > y2
            for i in 0..(self.start_x - self.end_x + 1) {
                covers.push(format!("{},{}", self.start_x - i, self.start_y - i));
            }
        }

        // covers.push(format!("{},{}", -2, -2));

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
        let count = input
            .split('\n')
            .map(|l| Line::from_input(l))
            .filter(|l| l.is_horizontal() || l.is_vertical() || l.is_diagonal())
            .flat_map(|l| l.covers_with_diags())
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

const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("example.txt");

fn main() {
    print!("Part 1: ");
    Part1::solve(INPUT);

    print!("Part 2: ");
    Part2::solve(INPUT);
}
