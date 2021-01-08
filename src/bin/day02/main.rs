use advent_of_code::puzzle::Puzzle;

struct Part1;
struct Part2;

#[derive(Debug)]
struct Entry {
    min: usize,
    max: usize,
    target: char,
    pass: String
}

impl Entry {

    fn new(entry: &str) -> Entry {
        let split_vec = entry.split(' ').collect::<Vec<&str>>();

        // min max parsed from
        // [1-2, (), ()] -> 1-2 -> (1, 2)
        let min_max_vec = split_vec[0].split('-').collect::<Vec<&str>>().into_iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let (min, max) = (min_max_vec[0], min_max_vec[1]);
    
        // inp: [(), r:, ()] -> r: -> r
        let target = split_vec[1].chars().nth(0).unwrap();
        
        Entry {
            min: min,
            max: max,
            target: target,
            pass: split_vec[2].to_string()
        }
    }

    fn char_at_pos(&self, pos: usize) -> char {
        self.pass.chars().nth(pos - 1).unwrap()
    }
}

impl Puzzle for Part1 {

    fn solve(input: &str) {
        let entries = input.split('\n').collect::<Vec<&str>>().into_iter()
            .map(|e| Entry::new(e))
            .collect::<Vec<Entry>>();

        let mut correct_pass = 0;
        for e in entries {
            // println!("{:?}", e);
            let count_char = e.pass.matches(e.target).count();

            if count_char <= e.max && count_char >= e.min {
                correct_pass = correct_pass + 1;
            }
        }

        println!("{}", correct_pass)
    }

}

impl Puzzle for Part2 {

    fn solve(input: &str) {
        let entries = input.split('\n').collect::<Vec<&str>>().into_iter()
        .map(|e| Entry::new(e))
        .collect::<Vec<Entry>>();

        let mut correct_pass = 0;
        for e in entries {
            if (e.char_at_pos(e.min) == e.target) ^ (e.char_at_pos(e.max) == e.target) {
                correct_pass += 1;
            }
        }

        println!("{}", correct_pass);
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