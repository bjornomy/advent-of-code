
trait Puzzle {
    fn solve(input: &str);
}
struct Part1;
struct Part2;

#[derive(Debug)]
struct Pair {
    start: usize,
    end: usize
}

impl Pair {
    fn from(s: &str) -> Self {
        let parts: Vec<usize> = s.split("-").into_iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        Pair {
            start: parts[0],
            end: parts[1]
        }
    }

    fn overlaps(&self, other: &Pair) -> bool {
        let o =  (self.start <= other.start && self.end >= other.end) || (other.start <= self.start && other.end >= self.end);
        //println!("{:?} <-> {:?}, overlaps: {:?}", self, other, o);
        return o;
    }

    fn overlaps2(&self, other: &Pair) -> bool {

        let o = (self.end >= other.start && self.start <= other.end) || (other.end >= self.start && other.start <= self.start);
        //println!("{:?} <-> {:?}, overlaps: {:?}", self, other, o);
        return o;
    }
}

impl Puzzle for Part1 {

    fn solve(input: &str) {
        let num_pars :usize = input.split('\n').collect::<Vec<&str>>().into_iter()
            .map(|row| {
                let pairs: Vec<Pair> = row.split(",").into_iter()
                    .map(Pair::from)
                    .collect();

                pairs[0].overlaps(&pairs[1])
            })
            .filter(|b| b == &true)
            .count();

        println!("{:?}", num_pars)
    }
}

impl Puzzle for Part2 {

    fn solve(input: &str) {
        let num_pars :usize = input.split('\n').collect::<Vec<&str>>().into_iter()
            .map(|row| {
                let pairs: Vec<Pair> = row.split(",").into_iter()
                    .map(Pair::from)
                    .collect();

                pairs[0].overlaps2(&pairs[1])
            })
            .filter(|b| b == &true)
            .count();

        println!("{:?}", num_pars)
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
