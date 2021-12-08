use advent_of_code_2021::puzzle::{self, Puzzle};

struct Part1;
struct Part2;

#[derive(Debug)]
struct LanternFish {
    life: i8,
}

impl Default for LanternFish {
    fn default() -> Self {
        LanternFish { life: 8 }
    }
}

impl LanternFish {
    fn from_str(input: &str) -> Self {
        let life: i8 = input.parse().unwrap();
        LanternFish { life: life }
    }

    fn tick(&mut self) -> bool {
        self.life -= 1;

        if self.life == -1 {
            self.life = 6;
            return true;
        } else {
            return false;
        }
    }
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        let mut fish: Vec<LanternFish> =
            input.split(',').map(|n| LanternFish::from_str(n)).collect();

        for _ in 0..80 {
            let mut new_fish: Vec<LanternFish> = Vec::new();

            for i in 0..fish.len() {
                if fish[i].tick() {
                    new_fish.push(LanternFish::default())
                }
            }
            fish.append(&mut new_fish);

            // println!("After {} days: {}", day, fish.len())
        }

        println!("{}", fish.len());

        let mut fish: [usize; 9] =
            input
                .split(',')
                .map(|n| n.parse().unwrap())
                .fold([0usize; 9], |mut acc, el: usize| {
                    acc[el] += 1;
                    acc
                });

        // println!("{:?}", fish);
        for day in 0..80 {
            fish[(day + 7) % 9] += fish[day % 9];
            //     println!(
            //         "After Day: {}, {:?} --> S: {}",
            //         day + 1,
            //         fish,
            //         fish.into_iter().sum::<usize>()
            //     )
        }
        println!("{}", fish.into_iter().sum::<usize>())
    }
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        let mut fish: [usize; 9] =
            input
                .split(',')
                .map(|n| n.parse().unwrap())
                .fold([0usize; 9], |mut acc, el: usize| {
                    acc[el] += 1;
                    acc
                });

        for day in 0..256 {
            fish[(day + 7) % 9] += fish[day % 9];
            // println!("{:?}", fish)
        }
        println!("{}", fish.into_iter().sum::<usize>())
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
