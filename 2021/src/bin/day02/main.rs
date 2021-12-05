use advent_of_code_2021::puzzle::Puzzle;

struct Part1;
struct Part2;

fn parse(line :&str) -> (&str, usize) {
    let split :Vec<&str> = line.split(' ').collect();

    (split[0], split[1].parse::<usize>().unwrap())
}

impl Puzzle for Part1 {

    fn solve(input: &str) {

        let mut depth :usize = 0;
        let mut horizontal :usize = 0;

        input.split('\n')
            .map(|l| parse(l))
            .for_each(|line| {
                match line.0 {
                    "up" => depth -= line.1,
                    "down" => depth += line.1,
                    "forward" => horizontal += line.1,
                    _ => ()
                }
            });

        println!("{} * {} = {}", depth, horizontal, depth * horizontal)
    }
}

impl Puzzle for Part2 {

    fn solve(input: &str) {
        let mut depth :usize = 0;
        let mut horizontal :usize = 0;
        let mut aim :usize = 0;

        input.split('\n')
            .map(|l| parse(l))
            .for_each(|line| {
                match line.0 {
                    "up" => aim -= line.1,
                    "down" => aim += line.1,
                    "forward" =>  {
                        horizontal += line.1;
                        depth += aim * line.1;
                    },
                    _ => ()
                }
            });

        println!("{} * {} = {}", depth, horizontal, depth * horizontal)
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
