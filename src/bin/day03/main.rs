use advent_of_code::puzzle::Puzzle;

struct Part1;
struct Part2;

const TREE: char = '#';

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .split('\n')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn count_trees(map: &Vec<Vec<char>>, right: usize) -> usize {
    count_trees_2(map, right, 1)
}

fn count_trees_2(map: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut num_trees = 0;

    while x < map.len() {
        // wrap at end of line
        if y >= map[x].len() {
            y = y - map[x].len();
        }

        // println!("Checking {:?}, pos {}", x, y);

        if map[x][y] == TREE {
            // println!("HIT");
            num_trees += 1;
        }

        y += right;
        x += down;
    }

    num_trees
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        let map = parse(input);
        println!("{}", count_trees(&map, 3));
    }
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        let map = parse(input);

        let mut routes: Vec<usize> = Vec::new();
        routes.push(count_trees(&map, 1));
        routes.push(count_trees(&map, 3));
        routes.push(count_trees(&map, 5));
        routes.push(count_trees(&map, 7));
        routes.push(count_trees_2(&map, 1, 2));

        println!("{}", routes.into_iter().product::<usize>());
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
