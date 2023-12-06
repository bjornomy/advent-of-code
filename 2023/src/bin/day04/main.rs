use std::collections::{HashMap, HashSet};
use std::ffi::c_ushort;
use std::ops::Deref;
use std::str::FromStr;

trait Puzzle {
    fn solve(input: &str);
}

struct Part1;

struct Part2;

#[derive(Debug)]
struct Card {
    num: usize,
    winning: Vec<usize>,
    ours: Vec<usize>,
}

#[derive(PartialEq, Debug)]
struct CardParseError;

fn parse_number_list(s: &str) -> Vec<usize> {
    s.trim().replace("  ", " ").split(' ')
        .map(|is| {
            if let Ok(n) = is.trim().parse() {
                n
            } else {
                println!("Failed to parse: '{}'", is);
                0
            }
        })
        .collect::<HashSet<usize>>()
        .into_iter()
        .collect()
}

impl FromStr for Card {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a: Vec<&str> = s.split(':').collect();

        if let Ok(num) = a[0].replace("Card", "").replace(' ', "").parse() {
            let b: Vec<&str> = a[1].trim().split('|').collect();
            let mut winning = parse_number_list(b[0]);
            let mut ours = parse_number_list(b[1]);

            winning.sort();
            ours.sort();

            Ok(Card { num, winning, ours })
        } else {
            eprintln!("FAILED to parse {}", a[0]);
            Err(CardParseError)
        }
    }
}

impl Card {
    fn num_matches(&self) -> usize {
        let mut count = 0;

        self.winning.iter().for_each(|n| {
            if self.ours.contains(n) {
                count += 1;
            }
        });

        count
    }

    fn calc_points(&self) -> usize {
        let count = self.num_matches();
        if count == 0 {
            0
        } else {
            2usize.pow(count.try_into().unwrap_or(0))
        }
    }
}

fn parse_cards(input: &str) -> Vec<Card> {
    input.lines()
        .map(Card::from_str)
        .filter_map(|c| c.ok())
        // .map(|c| {
        //     println!("C: {:?}, P: {}", c, c.calc_points());
        //     c
        // })
        .collect()
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        let cards: Vec<Card> = parse_cards(input);

        let sum: usize = cards.iter()
            .map(|c| c.calc_points())
            .sum();
        println!("{}", sum);
    }
}

fn recursive_card_count(match_map: &HashMap<usize, usize>, cards: &mut Vec<usize>, max: usize) {

    if cards.iter().sum::<usize>() == 0 {
        // no cards to spawn
        return;
    }

    // create a new array for the new cards, won from the cards given in params
    let mut new_cards = vec![0; cards.len()];

    // println!("I: {:?}", cards);

    // for each card, we "win" 'n' new cards, where the cards
    // we win is the 'n' next cards, eg: card[0] = (1, 4) => [2, 3, 4, 5]
    for c in 0..cards.len() {

        if cards[c] == 0 {
            // optimization: we don't need to spawns any new cards when
            // there is 0 of the given card
            continue;
        }

        // the key of match_map goes from 1 to 'max'
        if let Some(matches) = match_map.get(&(c + 1)) {
            // println!("Spawning [{}] cards for card [{}]", matches * cards[c], c + 1);

            // we need to spawn new cards for each card given
            for _ in 0..cards[c] {
                let mut i: usize = 1;
                while i <= *matches && c + i < max {
                    new_cards[c + i] += 1;
                    i += 1;
                }
            }
        }
    }

    // println!("N: {:?}", new_cards);

    if new_cards.iter().sum::<usize>() > 0 {
        recursive_card_count(match_map, new_cards.as_mut(), max);
        for i in 0..new_cards.len() {
            cards[i] += new_cards[i];
        }
    }

    // println!("E: {:?}", cards);
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        let cards = parse_cards(input);

        let match_map: HashMap<usize, usize> = cards.iter()
            .map(|c| (c.num, c.num_matches()))
            .collect();

        // println!("{:?}", match_map);

        let last_card = match_map.keys()
            .max()
            .unwrap_or(&0);

        // build the original array of cards
        // each index contains the number of copies we have of the given card
        // e.g arr[0] == num copies of card 1
        let mut num_cards: Vec<usize> = vec![1; cards.len()];

        recursive_card_count(&match_map, num_cards.as_mut(), *last_card);
        println!("{}", num_cards.iter().sum::<usize>())
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

// 13280, too low
// 41796, too high
// 43556, too high

// 1  2,  4,  8, 16
// 0, 3, 28, 55, 78
