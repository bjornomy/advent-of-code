use advent_of_code_2021::puzzle::Puzzle;

struct Part1;
struct Part2;

#[derive(Default, Clone)]
struct Count {
    zeroes: usize,
    ones: usize
}

impl Count {

    fn most_common(&self) -> char {
        if self.zeroes > self.ones {
            '0'
        } else {
            '1'
        }
    }

    fn least_common(&self) -> char {
        if self.zeroes > self.ones {
            '1'
        } else {
            '0'
        }
    }
}

fn do_count(lines: &Vec<&str>) -> Vec<Count> {

    let mut counts :Vec<Count> = Vec::with_capacity(12);
    for _ in 0..12 {
        counts.push(Count::default())
    }

    for l in lines {
        for (i, c) in l.chars().enumerate() {
            match c {
                '1' => counts[i].ones += 1,
                '0' => counts[i].zeroes += 1,
                _ => ()
            }
        }
    }

    counts
}

fn filter_lines(lines: Vec<&str>, target :char, col :usize) -> Vec<&str> {
    lines.into_iter()
        .filter(|l| l.chars().collect::<Vec<char>>()[col] == target)
        .collect()
}

impl Puzzle for Part1 { 

    fn solve(input: &str) {

        
        let lines = input.split('\n').collect::<Vec<&str>>();
        let counts = do_count(&lines);

        let gamma_rate = counts.clone().into_iter()
            .map(|c| c.most_common())
            .collect::<String>();

        let epsilon_rate = counts.into_iter()
            .map(|c| c.least_common())
            .collect::<String>();

        let gamma_decimal = usize::from_str_radix(&gamma_rate, 2).unwrap();
        let epsilon_decimal = usize::from_str_radix(&epsilon_rate, 2).unwrap();

        println!("{}", gamma_decimal * epsilon_decimal)
    }
}

impl Puzzle for Part2 {

    fn solve(input: &str) {
        let lines = input.split('\n').collect::<Vec<&str>>();
        
        let mut search = lines.clone();
        for i in 0..11 {
            let counts = do_count(&search);
            search = filter_lines(search, counts[i].most_common(), i);

            if search.len() == 1 {
                break;
            }
        }

        let o2 = search[0];

        search = lines.clone();
        for i in 0..11 {
            let counts = do_count(&search);
            search = filter_lines(search, counts[i].least_common(), i);

            if search.len() == 1 {
                break;
            }
        }

        let co2 = search[0];

        let o2_decimal = usize::from_str_radix(o2, 2).unwrap();
        let co2_decimal = usize::from_str_radix(co2, 2).unwrap();

        println!("{}", o2_decimal * co2_decimal)

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
