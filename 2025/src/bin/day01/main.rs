trait Puzzle {
    fn solve(input: &str);
}
struct Part1;
struct Part2;

#[derive(Debug, PartialEq, Clone)]
enum DIR {
    LEFT,
    RIGHT,
}

static START: i16 = 50;
static DIAL_SIZE: i16 = 100;

type DialMoveResult = (i16, i16);

#[derive(Clone, Debug)]
struct Move {
    dir: DIR,
    num_moves: i16,
}

impl Move {
    fn new(dir: DIR, moves: i16) -> Self {
        return Move {
            dir,
            num_moves: moves,
        };
    }
}

fn parse_move(l: &str) -> Move {
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
        panic!("What?");
    }

    return Move {
        dir,
        num_moves: num,
    };
}

fn move_dial(loc: i16, m: Move) -> DialMoveResult {
    let steps_to_move = m.num_moves % DIAL_SIZE;
    let full_revs = m.num_moves / DIAL_SIZE;

    let new_pos = match m.dir {
        DIR::LEFT => (loc - steps_to_move).rem_euclid(DIAL_SIZE),
        DIR::RIGHT => (loc + steps_to_move).rem_euclid(DIAL_SIZE),
    };

    // Count partial wrap/hit in the remainder (only when remainder > 0)
    let partial_wrap = if steps_to_move == 0 {
        0i16
    } else {
        match m.dir {
            DIR::LEFT => {
                // don't count if we started at 0
                if loc != 0 {
                    // either we wrapped (new_pos > loc) OR we landed exactly on 0
                    ((new_pos > loc) || (new_pos == 0)) as i16
                } else {
                    0i16
                }
            }
            // RIGHT: we wrapped if new_pos < loc (this already covers new_pos == 0 except when loc == 0)
            DIR::RIGHT => (new_pos < loc) as i16,
        }
    };

    (new_pos, full_revs + partial_wrap)
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        let mut loc = START;
        let mut count = 0;

        input.lines().for_each(|l| {
            let m = parse_move(l);
            let r = move_dial(loc, m);
            loc = r.0;

            if loc == 0 {
                // println!("Stopped at 0");
                count += 1;
            }
        });

        println!("{}", count)
    }
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        let mut loc = START;
        let mut count = 0i16;

        input.lines().for_each(|l| {
            let m = parse_move(l);
            let r = move_dial(loc, m);
            loc = r.0;

            count += r.1;
        });

        println!("{}", count);
    }
}

const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("example.txt");

fn main() {
    print!("Part 1: ");
    // Part1::solve(INPUT);

    print!("Part 2: ");
    Part2::solve(INPUT);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_dial_test() {
        let mut pos;
        let r = move_dial(50, Move::new(DIR::LEFT, 68));
        pos = r.0;
        assert_eq!(pos, 82);
        assert_eq!(r.1, 1);

        let r = move_dial(pos, Move::new(DIR::LEFT, 30));
        pos = r.0;
        assert_eq!(pos, 52);
        assert_eq!(r.1, 0);

        let r = move_dial(pos, Move::new(DIR::RIGHT, 48));
        pos = r.0;
        assert_eq!(pos, 0);
        assert_eq!(r.1, 1);

        let r = move_dial(pos, Move::new(DIR::RIGHT, 200));
        pos = r.0;
        assert_eq!(pos, 0);
        assert_eq!(r.1, 2);

        let r = move_dial(pos, Move::new(DIR::LEFT, 200));
        pos = r.0;
        assert_eq!(pos, 0);
        assert_eq!(r.1, 2);

        let r = move_dial(pos, Move::new(DIR::RIGHT, 50));
        pos = r.0;
        assert_eq!(pos, 50);
        assert_eq!(r.1, 0);

        let r = move_dial(pos, Move::new(DIR::RIGHT, 274));
        pos = r.0;
        assert_eq!(pos, 24);
        assert_eq!(r.1, 3);

        let r = move_dial(pos, Move::new(DIR::LEFT, 824));
        pos = r.0;
        assert_eq!(pos, 0);
        assert_eq!(r.1, 8);

        let r = move_dial(pos, Move::new(DIR::LEFT, 99));
        pos = r.0;
        assert_eq!(pos, 1);
        assert_eq!(r.1, 1);

        let r = move_dial(99, Move::new(DIR::LEFT, 99));
        assert_eq!(r.0, 0);
        assert_eq!(r.1, 0);

        let r = move_dial(0, Move::new(DIR::LEFT, 99 * 3));
        assert_eq!(r.0, 3);
        assert_eq!(r.1, 3);

        let r = move_dial(0, Move::new(DIR::RIGHT, 99 * 3));
        assert_eq!(r.0, 97);
        assert_eq!(r.1, 2);
    }

    #[test]
    fn find_revs() {
        let r = move_dial(50, Move::new(DIR::LEFT, 450));
        assert_eq!(r.0, 0);
        assert_eq!(r.1, 4);

        let t = r.1 + 1;
        assert_eq!(5, t);

        let r = move_dial(50, Move::new(DIR::RIGHT, 450));
        assert_eq!(r.0, 0);
        assert_eq!(r.1, 5);

        let t = r.1;
        assert_eq!(5, t);

        let r = move_dial(89, Move::new(DIR::RIGHT, 11));
        assert_eq!(r.0, 0);
        assert_eq!(r.1, 1);

        let r = move_dial(0, Move::new(DIR::LEFT, 2));
        assert_eq!(r.0, 98);
        assert_eq!(r.1, 1);

        let r = move_dial(50, Move::new(DIR::RIGHT, 1000));
        assert_eq!(r.0, 50);
        assert_eq!(r.1, 10);
    }

    // VERY close = 6561
    // not correct = 6067
    // not 5685
}
