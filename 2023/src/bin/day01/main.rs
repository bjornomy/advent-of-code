
trait Puzzle {
    fn solve(input: &str);
}

struct Part1;

struct Part2;

type Number<'n> = (&'n str, &'n str);

fn replace_in_string(string: &str, from_pos: usize, to_replace: &str, replacement: &str) -> String {
    format!(
        "{}{}",
        &string[..from_pos],
        string[from_pos..].replace(to_replace, replacement)
    )
}

fn find_string_number(chars: &str) -> Option<Number> {
    match chars {
        "on" => Some(("one", "1")),
        "tw" => Some(("two", "2")),
        "th" => Some(("three", "3")),
        "fo" => Some(("four", "4")),
        "fi" => Some(("five", "5")),
        "si" => Some(("six", "6")),
        "se" => Some(("seven", "7")),
        "ei" => Some(("eight", "8")),
        "ni" => Some(("nine", "9")),
        _ => None
    }
}

// build a sliding window that checks 2 and 2 chars,
// replaces wherever it finds a valid "numeric word"

fn process_line(string: &str) -> String {

    let mut i :usize = 0;
    let mut l = string.len();
    let mut w = string.to_string();

    while i + 2 < l {
        let window = &w[i..i + 2];
        // println!("i: {}, l: {}, w: {}", i, l, window);
        if let Some(n) = find_string_number(window) {
            // println!("NT: {:?}", n);
            w = replace_in_string(&w, i, n.0, n.1);
            // println!("T: {}", w);
            l = w.len();
        }

        i += 1
    }

    w
}

fn calculate_sum(input: &str) -> usize {
    input.lines()
        .map(|l| {
            println!("L: {:?}", l);
            l
        })
        .map(process_line)
        .map(|l| {
            println!("PL: {:?}", l);
            let numbers = l.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap_or(0))
                .collect::<Vec<u32>>();

            // println!("P: {:?}", numbers);
            let numbers =  format!("{}{}", numbers[0], numbers[numbers.len() - 1]);
            numbers
        })
        .map(|numeric_string| numeric_string.parse().unwrap_or(0))
        .map(|n| {
            println!("N: {:?}", n);
            n
        })
        .sum()
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        println!("Sum 1: {:?}", calculate_sum(input))
    }
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        println!("Sum 2: {:?}", calculate_sum(input))
    }
}

const INPUT: &str = include_str!("input.txt");
//const INPUT: &str = include_str!("example.txt");
// const INPUT: &str = include_str!("example_2.txt");

fn main() {
    print!("Part 1: ");
    // Part1::solve(INPUT);

    // TODO: too high result...
    print!("Part 2: ");
    Part2::solve(INPUT);
}

#[test]
fn replace_in_string_test() {
    let target = "two1nine";
    let res = replace_in_string(target, 0, "two", "2");
    assert_eq!(res, "21nine");
    let res = replace_in_string(&res, 2, "nine", "9");
    assert_eq!(res, "219")
}

#[test]
fn find_number_in_string() {
    let target = "two1nine";
    if let Some(n) = find_string_number(&target[..1]) {
        assert_eq!(n, ("two", "2"))
    }
}

#[test]
fn replace_in_windows() {
    let target = "two1nine";
    let res = process_line(target);
    assert_eq!(res, "219")
}