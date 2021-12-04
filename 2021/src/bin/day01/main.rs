use advent_of_code_2021::puzzle::Puzzle;

struct Part1;
struct Part2;

impl Puzzle for Part1 {

    fn solve(input: &str) {

        let numbers: Vec<usize> = input.split('\n').collect::<Vec<&str>>().into_iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let mut last :usize = 0;
        let mut result :usize = 0;
        numbers.into_iter().for_each(
            |n| {
                if last > 0 && n > last {
                    result += 1;
                }
                last = n;
            }
        );

        println!("{}", result)
    }
}

impl Puzzle for Part2 {

    fn solve(input: &str) {
        let numbers: Vec<usize> = input.split('\n').collect::<Vec<&str>>().into_iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let mut last :usize = 0;
        let mut result :usize = 0;

        for (i, n) in numbers.iter().enumerate() {
            if n + 2 >= len(numbers) {
                return;
            }
            let sum = numbers[n] + numbers[n + 1] + numbers[n + 2]

            if last > 0 && numbers[n] > last {
                result += 1;
            }
            last = i;
        }

        println!("{}", result)
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
