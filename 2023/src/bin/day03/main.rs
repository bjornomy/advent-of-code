use std::collections::HashMap;

trait Puzzle {
    fn solve(input: &str);
}

struct Part1;

struct Part2;

type Schematics = Vec<Vec<char>>;
static ASTERIX: char = '*';
static BLANK: char = '.';

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Symbol {
    i: usize,
    j: usize,
    symbol: char,
}

impl Symbol {
    fn from(map: &Schematics, i: usize, j: usize) -> Option<Self> {
        let c = map[i][j];
        if c != BLANK && !c.is_ascii_digit() {
            Some(Symbol { i, j, symbol: c })
        } else {
            None
        }
    }

    fn is_asterix(&self) -> bool {
        self.symbol == ASTERIX
    }
}

struct Visited {
    is_numeric: bool,
    adjacent_symbol: Option<Symbol>,
}

impl Visited {
    fn is_adjacent_asterix(&self) -> bool {
        if let Some(s) = &self.adjacent_symbol {
            s.is_asterix()
        } else {
            false
        }
    }
}

fn visit(map: &Schematics, i: usize, j: usize) -> Visited {
    let is_numeric = map[i][j].is_ascii_digit();

    let mut symbol = None;

    // we don't need to consider symbol adjacency for non-numerical chars
    if !is_numeric {
        return Visited {
            is_numeric,
            adjacent_symbol: symbol
        };
    }

    // check left
    if i > 0 && symbol.is_none() {
        symbol = Symbol::from(map, i - 1, j);
    }

    // check lef-up
    if i > 0 && j > 0 && symbol.is_none() {
        symbol = Symbol::from(map, i - 1, j - 1);
    }
    // check up
    if j > 0 && symbol.is_none() {
        symbol = Symbol::from(map, i, j - 1);
    }
    // check right-up
    if i + 1 < map.len() && j > 0 && symbol.is_none() {
        symbol = Symbol::from(map, i + 1, j - 1);
    }

    // check right
    if i + 1 < map.len() && symbol.is_none() {
        symbol = Symbol::from(map, i + 1, j);
    }

    // check right-down
    if i + 1 < map.len() && j + 1 < map[i].len() && symbol.is_none() {
        symbol = Symbol::from(map, i + 1, j + 1);
    }
    // check down
    if j + 1 < map[i].len() && symbol.is_none() {
        symbol = Symbol::from(map, i, j + 1);
    }
    // check left-down
    if i > 0 && j + 1 < map[i].len() && symbol.is_none() {
        symbol = Symbol::from(map, i - 1, j + 1);
    }

    Visited {
        is_numeric,
        adjacent_symbol: symbol
    }
}

fn parse_schematics(input: &str) -> Schematics {
    let mut map: Schematics = Vec::new();
    input.lines().enumerate().for_each(|(i, l)| {
        if map.get(i).is_none() {
            map.push(Vec::new())
        }
        l.chars().for_each(|c| map[i].push(c))
    });

    map
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        let map: Schematics = parse_schematics(input);

        let mut parts: Vec<usize> = Vec::new();
        let mut wp: String = String::new();
        let mut could_be_part = false;

        for i in 0..map.len() {
            for j in 0..map[i].len() {
                let v = visit(&map, i, j);

                if v.adjacent_symbol.is_some() {
                    could_be_part = true;
                }

                if v.is_numeric {
                    wp.push(map[i][j])
                } else {
                    if could_be_part {
                        parts.push(wp.parse().unwrap_or(0));
                    }
                    could_be_part = false;
                    wp = String::new();
                }
            }
        }

        let sum: usize = parts.iter().sum();
        println!("{}", sum)
    }
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        let map: Schematics = parse_schematics(input);


        let mut parts: HashMap<Symbol, Vec<usize>> = HashMap::new();

        let mut wp: String = String::new();
        let mut symbol: Option<Symbol> = None;

        for i in 0..map.len() {
            for j in 0..map[i].len() {
                let v = visit(&map, i, j);

                // indicate that we could create a part out of the current
                // accumulated part number
                if symbol.is_none() && v.adjacent_symbol.is_some() {
                    symbol = v.adjacent_symbol;
                    // println!("Found symbol: {:?}", symbol.unwrap());
                }

                if v.is_numeric {
                    wp.push(map[i][j]);
                    // println!("Found digit, wp: {}", wp);
                } else {
                    // if we encountered something that was not a digit
                    // we can assume the part number has ended, and we should
                    // consider to add it to the list of part numbers.
                    if symbol.is_some() && symbol.unwrap().is_asterix() {
                        let loc = Option::unwrap(symbol);

                        parts.entry(loc).or_default();

                        if let Some(v) = parts.get_mut(&loc) {
                            // println!("Pushing {} to {:?}", wp, loc);
                            v.push(wp.parse().unwrap_or(0));
                        }
                    }

                    if !wp.is_empty() {
                        // println!("Done building part number");
                        symbol = None;
                        wp = String::new();
                    }
                }
            }
        }

        let sum: usize = parts.values()
            .filter(|g| g.len() == 2)
            .map(|v| v[0] * v[1])
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
