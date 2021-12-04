use std::io;
use advent_of_code::puzzle::Puzzle;

struct Part1;
impl Puzzle for Part1 {
    
    fn solve(input: &str) {
    
        let numbers: Vec<usize> = input.split('\n').collect::<Vec<&str>>().into_iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
    
        // println!("{:?}", numbers);
    
    
        const TARGET: usize = 2020;
        const PART: usize = 2;
        const NUM_ADDENDS: usize = PART + 1;
    
        let mut addends: Vec<usize> = Vec::new();
        let num_inputs = numbers.len();
    
        let mut found: bool = false;
    
        let mut i = 0;
        while !found && i < num_inputs {
            let mut j = 0;
            while !found && j < num_inputs {
    
                if NUM_ADDENDS == 3 {
                    let mut k = 0;
                    
                    while !found && k < num_inputs {
                        let found_nums: Vec<usize> = vec![numbers[i], numbers[j], numbers[k]];
    
                        if found_nums.clone().into_iter().sum::<usize>() == TARGET {
                            found_nums.into_iter().for_each(|val| addends.push(val));
                            found = true;
                        }
    
                        k = k + 1;
                    }
                } else {
                    let found_nums: Vec<usize> = vec![numbers[i], numbers[j]];
                    
                    if found_nums.clone().into_iter().sum::<usize>() == TARGET {
                        found_nums.into_iter().for_each(|val| addends.push(val));
                        found = true;
                    }
                }
    
                j = j + 1;
            }
    
            i = i + 1;
        }
        println!("{}", addends.into_iter().product::<usize>());
    }
}

const INPUT: &str = include_str!("input.txt");

fn main() -> io::Result<()> {
    Part1::solve(INPUT);
    Ok(())
}
