use advent_of_code_2021::puzzle::{self, Puzzle};

use std::collections::HashMap;

struct Part1;
struct Part2;

const SEG_SIZE: usize = 10;
type Segment<'s> = [&'s str; SEG_SIZE];

#[derive(Debug)]
struct Data<'d> {
    segment: Segment<'d>,
    signals: Vec<&'d str>,
    output: Vec<&'d str>,
}

impl<'d> Data<'d> {
    fn from(line: &'d str) -> Self {
        let splits: Vec<&str> = line.split(" | ").collect();

        Data {
            segment: [""; SEG_SIZE],
            signals: splits[0].split(' ').collect(),
            output: splits[1].split(' ').collect(),
        }
    }

    fn parse_signals(&mut self) {
        // 0 -> 6 | - '1 -> 4 | - '4 -> 3 #
        // 1 -> 2
        // 2 -> 5 | - '1 -> 4 | - '4 -> 3 #
        // 3 -> 5 | - '1 -> 3 #
        // 4 -> 4
        // 5 -> 5 | - '1 -> 4 | - '4 -> 2 #
        // 6 -> 6 | - '1 -> 5 #
        // 7 -> 3
        // 8 -> 7
        // 9 -> 6 | - '1 -> 4 | - '4 -> 2 #

        for signal in &self.signals {
            if signal.len() == 2 {
                self.segment[1] = signal;
            }
            if signal.len() == 3 {
                self.segment[7] = signal;
            }
            if signal.len() == 4 {
                self.segment[4] = signal;
            }
            if signal.len() == 7 {
                self.segment[8] = signal;
            }
        }

        for signal in &self.signals {
            let signal_sub_one = signal.clone().replace(&self.segment[1], "");
            let signal_sub_four = signal.clone().replace(&self.segment[4], "");
            if signal_sub_one.len() == 3 {
                self.segment[3] = signal;
            }

            if signal_sub_one.len() == 5 {
                self.segment[6] = signal;
            }

            if signal.len() == 5 {
                if signal_sub_four.len() == 3 {
                    self.segment[2] = signal;
                }
                if signal_sub_four.len() == 2 {
                    self.segment[5] = signal
                }
            }

            if signal.len() == 6 {
                if signal_sub_four.len() == 3 {
                    self.segment[0] = signal;
                }
                if signal_sub_four.len() == 2 {
                    self.segment[9] = signal
                }
            }
        }
    }

    fn get_signal(&self) -> usize {
        let signal_to_num: HashMap<&str, usize> =
            self.segment
                .into_iter()
                .enumerate()
                .fold(HashMap::new(), |mut acc, (i, el)| {
                    acc.insert(el, i);
                    acc
                });

        self.output
            .clone()
            .into_iter()
            .map(|o| signal_to_num.get(&o).unwrap_or(&0))
            .fold(String::default(), |acc, el| format!("{}{}", acc, el))
            .parse::<usize>()
            .unwrap()
    }
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        let segments: Vec<&str> = input
            .split('\n')
            .flat_map(|l| {
                let splits: Vec<&str> = l.split(" | ").collect();

                let segments: Vec<&str> = splits[1].split(' ').collect();
                segments
            })
            .collect();

        let count = segments
            .into_iter()
            .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
            .count();

        println!("{}", count)
    }
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        let sum: usize = input
            .split('\n')
            .map(|l| {
                let mut d = Data::from(l);
                d.parse_signals();
                println!("{:?}", d);
                d.get_signal()
            })
            .sum();

        println!("{}", sum)
    }
}

// const INPUT: &str = include_str!("input.txt");
const INPUT: &str = include_str!("example.txt");

fn main() {
    print!("Part 1: ");
    Part1::solve(INPUT);

    print!("Part 2: ");
    Part2::solve(INPUT);
}
