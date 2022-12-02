use puzzle::puzzle::Puzzle;

struct Part1;
struct Part2;

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
enum Res {
    Win,
    Draw,
    Loss
}

impl Res {
    fn as_points(&self) -> usize {
        match self {
            Res::Win => 6,
            Res::Draw => 3,
            Res::Loss => 0
        }
    }

    fn parse(str :&str) -> Result<Self, &str> {
        match str {
            "X" => Ok(Res::Loss),
            "Y" => Ok(Res::Draw),
            "Z" => Ok(Res::Win),
            _ => Err("Invalid input")
        }
    }

    fn as_hand(&self, other :&Hand) -> Hand {
        match self {
            Res::Win => {
                match other {
                    Hand::Scissors => Hand::Rock,
                    Hand::Paper => Hand::Scissors,
                    Hand::Rock => Hand::Paper,
                }
            },
            Res::Draw => {
                match other {
                    Hand::Scissors => Hand::Scissors,
                    Hand::Paper => Hand::Paper,
                    Hand::Rock => Hand::Rock
                }
            },
            Res::Loss => {
                match other {
                    Hand::Scissors => Hand::Paper,
                    Hand::Paper => Hand::Rock,
                    Hand::Rock => Hand::Scissors,
                }
            }
        }
    }
}

impl Hand {

    fn evaluate(&self, other :Hand) -> usize {

        //println!("Playing {:?} against {:?}", self, other);
        let res = self.play(other);
        let points = res.as_points();
        let hand_points = self.as_points();

        //println!("The played hand gives {:?} points, the result {:?} gives {:?} points, totaling {:?}", hand_points, res, points, points + hand_points);

        points + hand_points
    }

    fn play(&self, other :Hand) -> Res {
        match self {
            Hand::Rock => {
                match other {
                    Hand::Scissors => Res::Win,
                    Hand::Rock => Res::Draw,
                    Hand::Paper => Res::Loss
                }
            },
            Hand::Paper => {
                match other {
                    Hand::Rock => Res::Win,
                    Hand::Paper => Res::Draw,
                    Hand::Scissors => Res::Loss
                }
            },
            Hand::Scissors => {
                match other {
                    Hand::Paper => Res::Win,
                    Hand::Scissors => Res::Draw,
                    Hand::Rock => Res::Loss
                }
            }
        }
    }

    fn parse(str :&str) -> Result<Self, &str> {
        match str {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err("Invalid input")
        }
    }

    fn as_points(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3
        }
    }
}

impl Puzzle for Part1 {

    fn solve(input: &str) {
        let points :usize = input.split('\n').collect::<Vec<&str>>().into_iter()
            .map(|s| {
                let other = Hand::parse(&s[0..1]).expect("Failed to parse other Hand");
                let me = Hand::parse(&s[2..]).expect("Failed to parse my Hand");
                me.evaluate(other)
            })
            .sum();

        println!("{:?}", points)
    }
}

impl Puzzle for Part2 {

    fn solve(input: &str) {

        let points :usize = input.split('\n').collect::<Vec<&str>>().into_iter()
            .map(|s| {
                let other = Hand::parse(&s[0..1]).expect("Failed to parse other Hand");
                let wanted = Res::parse(&s[2..]).expect("Failed to parse wanted Res");
                let me = wanted.as_hand(&other);
                me.evaluate(other)
            })
            .sum();

        println!("{:?}", points)
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
