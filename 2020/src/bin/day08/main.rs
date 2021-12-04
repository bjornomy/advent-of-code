use advent_of_code::puzzle::Puzzle;
use std::convert::{TryFrom, TryInto};

struct Part1;
struct Part2;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Acc(i16),
    Jmp(i16),
    Nop(i16),
}

fn to_ops(inp: &str) -> Vec<Operation> {
    inp.split("\n")
        .map(|line| {
            let words: Vec<&str> = line.split(" ").collect();

            println!("{:?}", words);

            let op = match &(words[1][0..1]) {
                "+" => 1,
                _ => -1,
            };
            let val: i16 = words[1][1..].parse().unwrap();

            match words[0] {
                "acc" => Operation::Acc(val * op),
                "jmp" => Operation::Jmp(val * op),
                _ => Operation::Nop(val * op),
            }
        })
        .collect()
}

#[derive(Debug)]
struct OperationResult {
    accum: i16,
    last: i16
}

fn process_ops(ops: &Vec<Operation>) -> OperationResult {
    let mut visited: Vec<i16> = Vec::new();

    let mut i: i16 = 0;
    let mut acc = 0;
    while i < ops.len().try_into().unwrap() {
        if visited.contains(&i) {
            break;
        }

        visited.push(i);

        match ops[usize::try_from(i).unwrap()] {
            Operation::Acc(a) => {
                //println!("ACC: {}", a);
                acc += a
            }
            Operation::Jmp(j) => {
                //println!("JMP: {}", j);
                i += j - 1
            }
            _ => (),
        };

        i += 1;
    }

    return OperationResult { accum: acc, last: *visited.last().unwrap() }
}

fn first_switchable(ops: &Vec<Operation>, min: i16) -> i16 {
    let mut found = 0;
    let mut i = min + 1;

    while i < ops.len() as i16 && found == 0 {
        let op = &ops[(usize::try_from(i).unwrap())];

        found = match op {
            Operation::Nop(_) | Operation::Jmp(_) => i,
            _ => 0
        };

        i += 1;
    }

    return found
}

fn switched_clone(ops: &Vec<Operation>, switch: i16) -> Vec<Operation> {
    let mut cloned = ops.to_vec();
    let u_switch = usize::try_from(switch).unwrap();

    cloned[u_switch] = match ops[u_switch] {
        Operation::Jmp(arg) => Operation::Nop(arg),
        Operation::Nop(arg) => Operation::Jmp(arg),
        Operation::Acc(arg) => Operation::Acc(arg)
    };

    cloned
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        let ops = to_ops(input);
        let res = process_ops(&ops);
        println!("{}", res.accum)
    }
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        let ops = to_ops(input);
        let mut switch = 0;
        
        let mut res = process_ops(&ops);
        while res.last < ops.len() as i16 - 1 {
            switch = first_switchable(&ops, switch);
            res = process_ops(&switched_clone(&ops, switch));
        }
        println!("{}", res.accum)
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
