use std::cmp::min;
use std::num::ParseIntError;

trait Puzzle {
    fn solve(input: &str);
}

struct Part1;

struct Part2;

type Draws = [usize; 3];
type Game = (usize, Vec<Draws>);

static MAX_RED: usize = 12;
static MAX_BLUE: usize = 14;
static MAX_GREEN: usize = 13;

fn line_to_tuple(line: &str) -> Result<Game, ParseIntError> {
    let splits: Vec<&str> = line.split(':').collect();
    let game = splits[0].replace("Game ", "");

    let game: usize = game.parse()?;
    let draws: Vec<&str> = splits[1].split(';').collect();

    // println!("G: {}, PD: {:?}", game, draws);

    let mut parsed_draws: Vec<Draws> = Vec::new();
    draws.into_iter().for_each(|d| {
        let mut acc: Draws = [0, 0, 0];

        d.split(',').for_each(|id| {
            let split: Vec<&str> = id.trim().split(' ').collect();
            // println!("pds: {:?}", split);

            let num: usize = split[0].parse().unwrap_or(0);
            match split[1] {
                "red" => acc[0] = num,
                "green" => acc[1] = num,
                "blue" => acc[2] = num,
                s => println!("Unhandled color: '{}'", s)
            }
        });

        parsed_draws.push(acc);
    });

    Ok((game, parsed_draws))
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        let sum: usize = input.lines()
            .map(line_to_tuple)
            .filter(|o| {
                if let Ok(t) = o {
                    t.1.iter().all(|d| d[0] <= MAX_RED && d[1] <= MAX_GREEN && d[2] <= MAX_BLUE)
                } else {
                    false
                }
            })
            .map(|t| t.unwrap().0)
            .sum();

        println!("{}", sum)
    }
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        let sum: usize = input.lines()
            .map(line_to_tuple)
            .map(|t| {
                let mut min: Draws = [0, 0, 0];
                if let Ok(g) = t {
                    g.1.iter().for_each(|d| {
                        if d[0] > min[0] {
                            min[0] = d[0]
                        }
                        if d[1] > min[1] {
                            min[1] = d[1]
                        }
                        if d[2] > min[2] {
                            min[2] = d[2]
                        }
                    });
                }
                // println!("{:?}", min);
                min
            })
            .map(|mins| mins[0] * mins[1] * mins[2])
            .sum();

        println!("{}", sum)
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