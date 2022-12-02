use puzzle::puzzle::Puzzle;

struct Part1;
struct Part2;

impl Puzzle for Part1 {

    fn solve(input: &str) {
        let mut current_cals :usize = 0;
        let mut elfs :Vec<usize> = Vec::new();

        for line in input.split('\n').into_iter() {
            if line.is_empty() {
                elfs.push(current_cals);
                current_cals = 0;
            } else {
                current_cals += line.parse::<usize>().unwrap()
            }
        }

        elfs.sort();
        if let Some(max) = elfs.last() {
            println!("{:?}", max);
        } else {
            println!("Could not find max_cals")
        }

    }
}

impl Puzzle for Part2 {

    fn solve(input: &str) {
        let mut current_cals :usize = 0;
        let mut elfs :Vec<usize> = Vec::new();

        for line in input.split('\n').into_iter() {
            if line.is_empty() {
                elfs.push(current_cals);
                current_cals = 0;
            } else {
                current_cals += line.parse::<usize>().unwrap()
            }
        }

        elfs.sort();
        let mut max :usize = 0;
        for i in (elfs.len() - 3)..elfs.len() {
            if let Some(cals) = elfs.get(i) {
                max += cals;
            }
        }

        println!("{:?}", max);
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
