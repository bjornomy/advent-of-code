use std::collections::HashMap;

trait Puzzle {
    fn solve(input: &str);
}
struct Part1;
struct Part2;

fn build_stacks(input: Vec<&str>) -> Vec<Vec<char>> {
    let num_stacks = input.last()
        .map(|s| s.replace(" ", ""))
        .map(|s| s.chars().last())
        .flatten()
        .map(|s| s.to_digit(10))
        .flatten()
        .expect("could not get or parse last row");

    let mut package_pos:usize = 1;
    let mut stacks: Vec<Vec<char>>= Vec::new();
    // should give array of 'stacks'
    for _ in 0..num_stacks {
        let mut stack_packages: Vec<char> = Vec::new();

        // need to push from bottom and up
        for j in (0..(input.len() - 1)).rev() {
            let chars: Vec<char> = input[j].chars().collect();
            if chars.len() > package_pos && chars[package_pos] != ' ' {
                stack_packages.push(chars[package_pos])
            }
        }

        stacks.push(stack_packages);
        package_pos += 4;
    }

    stacks
}

impl Puzzle for Part1 {

    fn solve(input: &str) {

        let parts = input.split("\n\n").collect::<Vec<&str>>();
        let mut stacks = build_stacks(parts[0].lines().into_iter().collect());

        parts[1].lines().into_iter()
            .for_each(|line| {
                let parts: Vec<&str> = line.split(" ").collect();
                let num_to_move = parts[1].parse::<usize>().expect("number");
                let from_stack = parts[3].parse::<usize>().expect("number") - 1;
                let to_stack = parts[5].parse::<usize>().expect("number") - 1;

                for _ in 0..num_to_move {
                    let package = stacks[from_stack].pop().expect("not empty");
                    stacks[to_stack].push(package);
                }
            });

        let sol: String = stacks.into_iter()
            .map(|stack| *stack.last().expect("element"))
            .collect();

        println!("{:?}", sol)
    }
}

impl Puzzle for Part2 {

    fn solve(input: &str) {
        let parts = input.split("\n\n").collect::<Vec<&str>>();
        let mut stacks = build_stacks(parts[0].lines().into_iter().collect());

        parts[1].lines().into_iter()
            .for_each(|line| {
                let parts: Vec<&str> = line.split(" ").collect();
                let num_to_move = parts[1].parse::<usize>().expect("number");
                let from_stack = parts[3].parse::<usize>().expect("number") - 1;
                let to_stack = parts[5].parse::<usize>().expect("number") - 1;

                let mut packages: Vec<char> = Vec::new();
                for _ in 0..num_to_move {
                    packages.push(stacks[from_stack].pop().expect("some value"))
                }

                for p in packages.into_iter().rev() {
                    stacks[to_stack].push(p);
                }
            });

        let sol: String = stacks.into_iter()
            .map(|stack| *stack.last().expect("element"))
            .collect();

        println!("{:?}", sol)
    }
}

const INPUT: &str = include_str!("input.txt");
//const INPUT: &str = include_str!("example.txt");

fn main() {
    print!("Part 1: ");
    Part1::solve(INPUT);

    print!("Part 2: ");
    Part2::solve(INPUT); 
}
