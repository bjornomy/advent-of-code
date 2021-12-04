use advent_of_code::puzzle::Puzzle;
use std::collections::{ HashSet, HashMap };
use std::convert::TryFrom;

struct Part1;
struct Part2;

fn persons_in_groups(inp: &str) -> Vec<Vec<&str>> {
    let lines: Vec<&str> = inp.split('\n').collect();
    let mut groups: Vec<Vec<&str>> = Vec::new();
    groups.push(Vec::new());

    let mut group_cnt = 0;

    for line in lines {
        if line.len() == 0 {
            groups.push(Vec::new());
            group_cnt += 1;
        } else {
            groups[group_cnt].push(line)
        }
    }

    //println!("G: {:?}", groups);

    return groups;
}

fn uniqe_answers(persons: &Vec<&str>) -> HashSet<char> {
    let mut chars = HashSet::new();
    for person in persons {
        person.chars().for_each(|c| {
            chars.insert(c);
            ()
        });
    }

    return chars;
}

fn answers(persons: &Vec<&str>) -> HashSet<char> {
    let mut anw: HashMap<char, i8> = HashMap::new();

    for person in persons {
        person.chars().for_each(|c| {
            if anw.contains_key(&c) {
                anw.insert(c, anw.get(&c).unwrap() +  1);
            } else {
                anw.insert(c, 1);
            }
        })
    }

    let chars: HashSet<char> = anw.iter()
        .filter(|(_, v)| usize::try_from(**v).unwrap() == persons.len())
        .map(|(k, _)| *k)
        .collect();

    return chars;
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        let sum: usize = persons_in_groups(input)
            .iter()
            .map(|g| uniqe_answers(g))
            .map(|hs| hs.len())
            .sum();

        println!("{}", sum)
    }
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        let sum: usize = persons_in_groups(input)
            .iter()
            .map(|g| answers(g))
            .map(|hs| hs.len())
            .sum();

        println!("{}", sum)
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
