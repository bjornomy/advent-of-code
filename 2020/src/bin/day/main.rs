use advent_of_code::puzzle::Puzzle;

struct Part1;
struct Part2;

impl Puzzle for Part1 {

    fn solve(_input: &str) {

    }
}

impl Puzzle for Part2 {

    fn solve(_input: &str) {

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
