use advent_of_code::puzzle::Puzzle;

struct Part1;
struct Part2;

const ROWS: u8 = 128;
const SEATS: u8 = 8;

/// max: 127, min: 0
/// op: f
/// max: 63, min 0
/// op: b
/// max: 63, min 32
/// op: b
/// max: 63, min 48
/// op:f
/// max: 63 - ((63 + 1 - 48) / 2) = 56, min: 48
/// op:b
/// max 55, min 48 + (55 + 1 - 48) / 2 = 52
/// op: f
/// max: 55 - (55 + 1 - 52) / 2 = 55 - 2 = 53, min 52
/// op b
/// 53 

fn binary_search(inp: &str, min: u8, max: u8) -> Option<u8> {

    if inp.len() > 1 {
        match &inp[0..1] {
            "F" | "L" => {
                // 63 - (63 + 1 - 32) / 2 = 63 - 8 = 55
                let max = max - min_max_change(min, max);
                binary_search(&inp[1..], min, max)
            },
            "B" | "R" => {
                // 32 + (63 + 1 - 32) / 2 = 32 + 16 = 48
                let min = min + min_max_change(min, max);
                binary_search(&inp[1..], min, max)
            }
            _ => None
        }
    } else {
        match inp {
            "F" | "L" => Some(min),
            "B" | "R" => Some(max),
            _ => None 
        }
    }
}
fn min_max_change(min: u8, max: u8) -> u8 {
    (max + 1 - min) / 2
}

fn to_lines(inp: &str) -> Vec<&str> {
    inp.split('\n').collect()
}

impl Puzzle for Part1 {

    fn solve(input: &str) {

        let lines = to_lines(input);

        let mut highest = 0;
        for line in lines {
            let rowcmds = &line[..7];
            let seatcmds = &line[7..];

            let row = binary_search(rowcmds, 0, ROWS - 1);
            let seat = binary_search(seatcmds, 0, SEATS - 1);

            match (row, seat) {
                (Some(row), Some(seat)) => {

                    let row: u16 = row.into();
                    let seat: u16 = seat.into();
                    let uq_seat: u16 = row * 8 + seat;

                    if uq_seat  > highest {
                        highest = uq_seat
                    }
                }
                _ => println!("Failed")
            }
        }

        println!("{}", highest)
    }
}

impl Puzzle for Part2 {

    fn solve(input: &str) {

        let mut seat_ids: Vec<u16> = Vec::new();

        for line in to_lines(input) {
            let rowcmds = &line[..7];
            let seatcmds = &line[7..];

            let row = binary_search(rowcmds, 0, ROWS - 1);
            let seat = binary_search(seatcmds, 0, SEATS - 1);

            match (row, seat) {
                (Some(row), Some(seat)) => {

                    let row: u16 = row.into();
                    let seat: u16 = seat.into();

                    seat_ids.push(row * 8 + seat);
                }
                _ => println!("Failed")
            }
        }

        seat_ids.sort();

        for x in 1..seat_ids.len() - 1 {
            
            // if there is a seatid where the next in the list are + 2 (instead of + 1)
            // seatid + 1 should be empty

            let t = seat_ids[x];

            if t + 2 == seat_ids[x + 1] {
                println!("{}", t + 1)
            }
        }
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
