trait Puzzle {
    fn solve(input: &str);
}
struct Part1;
struct Part2;

#[derive(Debug)]
enum DIR {
    LEFT,
    RIGHT,
}

static START: i16 = 50;
static MAX_DIAL: i16 = 99;

// actual change = change % 99
// depending on how many times we pass 0, something has to be subtraced from change
// as this definition doesn't count the postition '0' as 'step'

fn move_dial(loc: i16, dir: DIR, num_moves: i16) -> i16 {
    let revs = num_moves / MAX_DIAL;
    let actual_change = num_moves % MAX_DIAL;

    println!(
        "Moving dial in dir: {:?} {} steps. loc: {}, change: {}, revs: {}",
        dir, num_moves, loc, actual_change, revs
    );

    let mut computed = loc;

    match dir {
        DIR::LEFT => computed -= actual_change - revs,
        DIR::RIGHT => computed += actual_change - revs,
    }

    if computed < 0 {
        computed += MAX_DIAL + 1;
        println!("Passed 0, adding. New computed: {}", computed);
    } else if computed > MAX_DIAL {
        computed -= MAX_DIAL + 1;
        println!("Passed 99, subtracing. New loc: {}", computed)
    } else {
        println!("ended at loc: {}", computed);
    }

    return computed;
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        let mut loc = START;
        let mut count = 0u16;

        input.lines().for_each(|l| {
            let num: i16;
            let dir;
            // println!("Parsing line: {}", l);
            if l.starts_with("L") {
                num = l
                    .strip_prefix("L")
                    .expect("should remove prefix")
                    .parse()
                    .expect("valid number");
                dir = DIR::LEFT;
            } else if l.starts_with("R") {
                num = l
                    .strip_prefix("R")
                    .expect("should remove prefix")
                    .parse()
                    .expect("valid number");
                dir = DIR::RIGHT;
            } else {
                !panic!("What?");
            }

            loc = move_dial(loc, dir, num);

            if loc == 0 {
                println!("Stopped at 0? loc: {}", loc);
                count += 1;
            }
        });

        println!("C: {}", count)
    }
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        println!("NOT STARTED")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_dial_test() {
        let mut pos = move_dial(50, DIR::LEFT, 68);
        assert_eq!(pos, 82);

        pos = move_dial(pos, DIR::LEFT, 30);
        assert_eq!(pos, 52);

        pos = move_dial(pos, DIR::RIGHT, 48);
        assert_eq!(pos, 0);

        pos = move_dial(pos, DIR::RIGHT, 200);
        assert_eq!(pos, 0);

        pos = move_dial(pos, DIR::LEFT, 200);
        assert_eq!(pos, 0);

        pos = move_dial(pos, DIR::RIGHT, 50);
        assert_eq!(pos, 50);

        pos = move_dial(pos, DIR::RIGHT, 274);
        assert_eq!(pos, 24);

        pos = move_dial(pos, DIR::LEFT, 824);
        assert_eq!(pos, 0);

        pos = move_dial(pos, DIR::LEFT, 99);
        assert_eq!(pos, 1);

        pos = move_dial(0, DIR::LEFT, 99 * 3);
        assert_eq!(pos, 3);

        pos = move_dial(0, DIR::RIGHT, 99 * 3);
        assert_eq!(pos, 97);
    }
}
