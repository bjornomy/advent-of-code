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
static MAX_DIAL: i16 = 99;

type DialMoveResult = (i16, i16);

#[derive(Clone)]
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
        !panic!("What?");
    }

    return Move {
        dir,
        num_moves: num,
    };
}

fn move_dial_2(loc: i16, m: Move) -> DialMoveResult {
    println!(
        "Moving dial {:?} {} steps from: {}",
        m.dir, m.num_moves, loc
    );

    let mut revs = 0;
    let mut calc = m.num_moves;
    while calc > 100 {
        calc -= 100;
        revs += 1;
    }

    let mut next: i16;
    let mut wrapped = false;
    match m.dir {
        DIR::LEFT => {
            next = loc - calc;
            if next < 0 {
                next += 100;
                wrapped = true;
            }
        }
        DIR::RIGHT => {
            next = loc + calc;
            if next >= 100 {
                next -= 100;
                wrapped = true;
            }
        }
    }

    let mut wrap_zero_count = revs;
    if wrapped {
        wrap_zero_count += 1;
    }

    (next, wrap_zero_count)
}

fn _move_dial(loc: i16, dir: &DIR, num_moves: i16) -> DialMoveResult {
    let actual_change = num_moves % MAX_DIAL;

    let revs;
    if num_moves == MAX_DIAL && loc == MAX_DIAL {
        revs = 0;
    } else {
        revs = num_moves / MAX_DIAL
    }

    let mut wrap_zero_count = revs;

    println!(
        "Moving dial {:?} {} steps from: {}, change: {}, revs: {}",
        dir, num_moves, loc, actual_change, revs
    );

    let mut computed = loc;

    match dir {
        DIR::LEFT => {
            computed -= actual_change;
            computed += revs;
        }
        DIR::RIGHT => {
            computed += actual_change;
            computed -= revs;
        }
    }

    let mut wrapped = false;
    if computed < 0 {
        computed += MAX_DIAL + 1;
        // println!("Passed 0, adding. New computed: {}", computed);
        wrapped = true;
    } else if computed > MAX_DIAL {
        computed -= MAX_DIAL + 1;
        // println!("Passed 99, subtracing. New loc: {}", computed)
        wrapped = true;
    } else {
        // println!("ended at loc: {}", computed);
    }

    if loc != 0 && wrapped {
        wrap_zero_count += 1;
    }

    println!("Wrapped 0: {}", wrap_zero_count);

    return (computed, wrap_zero_count);
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        let mut loc = START;
        let mut count = 0;

        input.lines().for_each(|l| {
            let m = parse_move(l);
            let r = move_dial_2(loc, m);
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
            let orig = loc;
            let r = move_dial_2(loc, m.clone());
            loc = r.0;

            if m.dir == DIR::LEFT {
                if r.0 == 0 && r.1 > 1 {
                    count += 1;
                }
                if orig == 0 {
                    count -= 1;
                }
            }

            println!("Cur wrap: {}, adding: {}", count, r.1);
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
        let r = move_dial_2(50, Move::new(DIR::LEFT, 68));
        pos = r.0;
        assert_eq!(pos, 82);
        assert_eq!(r.1, 1);

        let r = move_dial_2(pos, Move::new(DIR::LEFT, 30));
        pos = r.0;
        assert_eq!(pos, 52);
        assert_eq!(r.1, 0);

        let r = move_dial_2(pos, Move::new(DIR::RIGHT, 48));
        pos = r.0;
        assert_eq!(pos, 0);
        assert_eq!(r.1, 1);

        let r = move_dial_2(pos, Move::new(DIR::RIGHT, 200));
        pos = r.0;
        assert_eq!(pos, 0);
        assert_eq!(r.1, 2);

        let r = move_dial_2(pos, Move::new(DIR::LEFT, 200));
        pos = r.0;
        assert_eq!(pos, 0);
        assert_eq!(r.1, 2);

        let r = move_dial_2(pos, Move::new(DIR::RIGHT, 50));
        pos = r.0;
        assert_eq!(pos, 50);
        assert_eq!(r.1, 0);

        let r = move_dial_2(pos, Move::new(DIR::RIGHT, 274));
        pos = r.0;
        assert_eq!(pos, 24);
        assert_eq!(r.1, 3);

        let r = move_dial_2(pos, Move::new(DIR::LEFT, 824));
        pos = r.0;
        assert_eq!(pos, 0);
        assert_eq!(r.1, 8);

        let r = move_dial_2(pos, Move::new(DIR::LEFT, 99));
        pos = r.0;
        assert_eq!(pos, 1);
        assert_eq!(r.1, 1);

        let r = move_dial_2(99, Move::new(DIR::LEFT, 99));
        assert_eq!(r.0, 0);
        assert_eq!(r.1, 0);

        let r = move_dial_2(0, Move::new(DIR::LEFT, 99 * 3));
        assert_eq!(r.0, 3);
        assert_eq!(r.1, 3);

        let r = move_dial_2(0, Move::new(DIR::RIGHT, 99 * 3));
        assert_eq!(r.0, 97);
        assert_eq!(r.1, 2);
    }

    #[test]
    fn find_revs() {
        let r = move_dial_2(50, Move::new(DIR::LEFT, 450));
        assert_eq!(r.0, 0);
        assert_eq!(r.1, 4);

        let t = r.1 + 1;
        assert_eq!(5, t);

        let r = move_dial_2(50, Move::new(DIR::RIGHT, 450));
        assert_eq!(r.0, 0);
        assert_eq!(r.1, 5);

        let t = r.1;
        assert_eq!(5, t);

        let r = move_dial_2(89, Move::new(DIR::RIGHT, 11));
        assert_eq!(r.0, 0);
        assert_eq!(r.1, 1);

        let r = move_dial_2(0, Move::new(DIR::LEFT, 2));
        assert_eq!(r.0, 98);
        assert_eq!(r.1, 1);

        let r = move_dial_2(50, Move::new(DIR::RIGHT, 1000));
        assert_eq!(r.0, 50);
        assert_eq!(r.1, 10);
    }

    // VERY close = 6561
    // not correct = 6067
    // not 5685
}
