use advent_of_code::puzzle::Puzzle;
use regex::Regex;
use std::collections::HashMap;

struct Part1;
struct Part2;

const MIN_KEYS: usize = 7;
const CID_KEY: &str = "cid";

fn get_key_value(inp: &str) -> (&str, &str) {
    let vec = inp.split(':').collect::<Vec<&str>>();

    (vec[0], vec[1])
}

fn validate_pp(map: &HashMap<&str, &str>) -> bool {
    let byr = validate_year(map.get("byr").unwrap(), 1920, 2002);
    let iyr = validate_year(map.get("iyr").unwrap(), 2010, 2020);
    let eyr = validate_year(map.get("eyr").unwrap(), 2020, 2030);

    let hgt = map.get("hgt").unwrap();

    let hgt = validate_height(hgt);

    let hcl = Regex::new(r"^#[0-9a-f]{6}$")
        .unwrap()
        .is_match(map.get("hcl").unwrap());

    let ecl = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$")
        .unwrap()
        .is_match(map.get("ecl").unwrap());

    let pid = Regex::new(r"[0-9]{9}")
        .unwrap()
        .is_match(map.get("pid").unwrap());

    byr && iyr && eyr && hgt && hcl && ecl && pid
}

fn validate_height(hgt: &str) -> bool {
    let hr = Regex::new(r".*(cm|in)").unwrap();

    // invalid if not matching
    if !hr.is_match(hgt) {
        return false;
    } else {
        // all chars except the last 2
        let hgt_num = hgt[..(hgt.len() - 2)].parse::<usize>().unwrap();

        // should find the last 2 chars and match them
        return match &hgt[(hgt.len() - 2)..] {
            "cm" => validate_num(hgt_num, 150, 193),
            "in" => validate_num(hgt_num, 59, 76),
            _ => false,
        };
    }
}

fn validate_year(year: &str, least: usize, most: usize) -> bool {
    let year_num = year.parse::<usize>().unwrap();
    year.len() == 4 && validate_num(year_num, least, most)
}

fn validate_num(num: usize, least: usize, most: usize) -> bool {
    num >= least && num <= most
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        let lines = input.split('\n').collect::<Vec<&str>>();

        let mut km: HashMap<&str, &str> = HashMap::new();
        let mut valid_pps = 1;

        print!(
            "Total pps: {} : ",
            lines
                .clone()
                .into_iter()
                .filter(|l| l.len() == 0)
                .collect::<Vec<&str>>()
                .len()
        );

        for l in lines {
            if l.len() == 0 {
                if km.keys().count() >= MIN_KEYS {
                    valid_pps += 1;
                }
                // we've processed 1 pp, clear for new one
                km.clear();
            } else {
                // handle multiple key_value on same line, ie:
                // key:val key:val ...
                let values_on_line = l.split(' ').collect::<Vec<&str>>();

                for elem in values_on_line {
                    let kv = get_key_value(elem);

                    // we don't care about cid
                    if kv.0 != CID_KEY {
                        km.insert(kv.0, kv.1);
                    }
                }
            }
        }

        println!("{}", valid_pps)
    }
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        let lines = input.split('\n').collect::<Vec<&str>>();

        let mut km: HashMap<&str, &str> = HashMap::new();
        let mut valid_pps = 0;

        print!(
            "Total pps: {} : ",
            lines
                .clone()
                .into_iter()
                .filter(|l| l.len() == 0)
                .collect::<Vec<&str>>()
                .len()
        );

        for l in lines {
            if l.len() == 0 {
                if km.keys().count() >= MIN_KEYS && validate_pp(&km) {
                    valid_pps += 1;
                }

                km.clear();
            } else {
                // handle multiple key_value on same line, ie:
                // key:val key:val...
                let values_on_line = l.split(' ').collect::<Vec<&str>>();

                for elem in values_on_line {
                    let kv = get_key_value(elem);
                    // we don't care about cid
                    if kv.0 != CID_KEY {
                        km.insert(kv.0, kv.1);
                    }
                }
            }
        }

        println!("{}", valid_pps)
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
