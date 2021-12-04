use advent_of_code::puzzle::Puzzle;
use std::collections::HashMap;
use std::convert::TryFrom;

struct Part1;
struct Part2;

fn content_of_bag(line: &str) -> HashMap<String, u8> {
    let mut m = HashMap::new();

    let words: Vec<&str> = line.split(' ').collect();

    // words[0] == color_mod
    // words[1] == color
    // word[2] == bag[s]?
    // word[3] == contains
    let mut i = 4;
    while i < words.len() {
        // words[i] == num bags
        // words[i + 1] == color mod
        // words[i + 2] == color
        let num = words[i];
        let key = words[i + 1].to_owned() + " " + words[i + 2];

        // words [i + 3] == bag[s]?(,|.)

        if num == "no" {
            m.insert("NOPE".to_string(), 1);
        } else {
            m.insert(key, num.parse().unwrap());
        }

        i += 4
    }

    m
}

fn bags_holding(map: &HashMap<String, HashMap<String, u8>>, bag: &str) -> Vec<String> {
    let mut bags: Vec<String> = Vec::new();

    map.iter()
        .filter(|(_, v)| v.contains_key(bag))
        .for_each(|(k, _)| {
            bags.push(k.to_string());
            ()
        });

    bags
}

fn find_holding_bags(map: &HashMap<String, HashMap<String, u8>>, bags: Vec<String>) -> Vec<String> {
    bags.iter().map(|bag| {
            let bh = bags_holding(map, bag);

            let mut vec = vec![bag.clone()];

            vec.append(&mut find_holding_bags(map, bh));

            vec.sort();
            vec.dedup();

            // println!("Appended vec: {:?}", vec);

            return vec;
        })
        .flatten()
        .collect()
}


// map a -> | 2b | 4c 

fn count_bags(map: &HashMap<String, HashMap<String, u8>>, bags: Vec<String>, indent: u8) -> usize {

    bags.iter().map(|bag| {
        println!("");

        for _ in 0..indent {
            print!("- ");
        }

        print!("b: {}, m: {:?}", bag, map.get(bag));

        match map.get(bag) {

            Some(inner_map) => {
                let sum = inner_map.iter().map(|(k, v)| {
                    // print!("\n- {}, {}", k, v);
                    let usize_k = usize::try_from(*v).unwrap();

                    if k == "- NOPE" {
                        usize_k
                    } else {
                        usize_k * count_bags(map, vec![k.clone()], indent + 1)
                    }
                })
                .sum::<usize>();

                println!("{}", sum);
                return sum;
            },
            None => 1
        }
    })
    .sum()
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        let mut bag_to_content: HashMap<String, HashMap<String, u8>> = HashMap::new();

        input
            .split('\n')
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|line| {
                let words: Vec<&str> = line.split(' ').collect();
                let key = words[0].to_string() + " " + words[1];
                bag_to_content.insert(key, content_of_bag(line));
                ()
            });

        // find all that holds bag directly
        // for each found, find those who hold those
        // repeat untill no bags found
        // acc list of unique color_mod + color.

        let inital_bags = bags_holding(&bag_to_content, TARGET_BAG);
        let mut bags = find_holding_bags(&bag_to_content, inital_bags);

        bags.sort();
        bags.dedup();

        // println!("{:?}", bags);
        println!("{:?}", bags.len());
    }
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        let mut bag_to_content: HashMap<String, HashMap<String, u8>> = HashMap::new();

        input
            .split('\n')
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|line| {
                let words: Vec<&str> = line.split(' ').collect();
                let key = words[0].to_string() + " " + words[1];
            
                bag_to_content.insert(key, content_of_bag(line));
                ()
            });

        // find which bags target contains
        // recusively build some kind of map structure to keep the bag counts
        // bag1 -> 2x bag2 && 5x bag3 | bag2 -> 3x bag4 | bag4 -> 0 | bag3 -> 0

        let mut tot_sum = count_bags(&bag_to_content, vec![TARGET_BAG.to_string()], 0);

        match bag_to_content.get(TARGET_BAG) {
             Some(bag) => tot_sum += bag.values().map(|n| usize::try_from(*n).unwrap()).sum::<usize>(),
             None => ()
        };

        println!("{}", tot_sum)
    }
}

const TARGET_BAG: &str = "shiny gold";

const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("example.txt");

fn main() {
    print!("Part 1: ");
    Part1::solve(INPUT);

    print!("Part 2: ");
    Part2::solve(INPUT);
}

// 1(3*1+4*1) + 2(5*1+6*1) = 1*7 + 2*11 = 28