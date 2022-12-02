#! /bin/sh

day=$(date +'%d')

path="$1/src/bin/day$day"
mkdir -p $path

touch $path/input.txt
touch $path/example.txt
cat > $path/main.rs <<- EOM
use puzzle::puzzle::Puzzle;

struct Part1;
struct Part2;

impl Puzzle for Part1 {

    fn solve(input: &str) {
        println!("NOT STARTED")
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
EOM
