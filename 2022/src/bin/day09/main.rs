use std::collections::HashSet;

trait Puzzle {
    fn solve(input: &str);
}

struct Part1;

struct Part2;

// 9 possible pos around head (including heads position itself)
// move tail closer whenever head is too far away
// if tail outside the 9, move cardinal/diag towards head

// y
// | (x, y)
// \_______ x

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Bridge {
    knots: Vec<Point>,
    visited: HashSet<Point>,
}

impl Default for Bridge {
    fn default() -> Self {
        Bridge {
            knots: vec![Point::default(), Point::default()],
            visited: HashSet::from([Point::default()]),
        }
    }
}

enum Move {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32),
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let splits: Vec<&str> = s.split(' ').collect();

        let distance: i32 = splits[1].parse().expect("digit");

        match splits[0] {
            "R" => Move::Right(distance),
            "L" => Move::Left(distance),
            "U" => Move::Up(distance),
            "D" => Move::Down(distance),
            _ => unimplemented!()
        }
    }
}

impl Bridge {
    fn do_move(&mut self, mve: &Move) {
        let dist = match mve {
            Move::Right(dist) | Move::Left(dist) | Move::Up(dist) | Move::Down(dist) => *dist
        };

        for _ in 0..dist {
            self.move_head(mve);
            for i in 1..self.knots.len() {
                self.move_tail(i - 1, i);
            }
        }
    }

    fn move_head(&mut self, mve: &Move) {
        match mve {
            Move::Right(_) => self.knots[0].x += 1,
            Move::Left(_) => self.knots[0].x -= 1,
            Move::Up(_) => self.knots[0].y += 1,
            Move::Down(_) => self.knots[0].y -= 1
        }
    }

    fn move_tail(&mut self, head: usize, tail: usize) {
        if self.is_touching(head, tail) {
            return;
        }

        // diagonals
        //    R-U  |   R-D   |   L-U   |   L-D
        // .H  ..H | T.  T.. | H.  H.. | .T  ..T
        // ..  T.. | ..  ..H | ..  ..T | ..  H..
        // T.      | .H      | .T      | H.

        if (self.knots[tail].y == self.knots[head].y - 2 && self.knots[tail].x < self.knots[head].x)
            || (self.knots[tail].x == self.knots[head].x - 2 && self.knots[tail].y < self.knots[head].y)
        {                                                        // move right, up
            self.knots[tail].x += 1;
            self.knots[tail].y += 1;
        } else if (self.knots[tail].y == self.knots[head].y + 2 && self.knots[tail].x < self.knots[head].x)
            || (self.knots[tail].x == self.knots[head].x - 2 && self.knots[tail].y > self.knots[head].y)
        {                                                        // move right, down
            self.knots[tail].x += 1;
            self.knots[tail].y -= 1;
        } else if (self.knots[tail].y == self.knots[head].y - 2 && self.knots[tail].x > self.knots[head].x)
            || (self.knots[tail].x == self.knots[head].x + 2 && self.knots[tail].y < self.knots[head].y)
        {                                                        // move left, up
            self.knots[tail].x -= 1;
            self.knots[tail].y += 1;
        } else if (self.knots[tail].y == self.knots[head].y + 2 && self.knots[tail].x > self.knots[head].x)
            || (self.knots[tail].x == self.knots[head].x + 2 && self.knots[tail].y > self.knots[head].y)
        {                                                        // move left, down
            self.knots[tail].x -= 1;
            self.knots[tail].y -= 1;
        } else if self.knots[tail].x == self.knots[head].x - 2 { // move towards right
            self.knots[tail].x += 1;
        } else if self.knots[tail].x == self.knots[head].x + 2 { // move towards left
            self.knots[tail].x -= 1;
        } else if self.knots[tail].y == self.knots[head].y - 2 { // move upwards
            self.knots[tail].y += 1;
        } else if self.knots[tail].y == self.knots[head].y + 2 { // move downwards
            self.knots[tail].y -= 1;
        }

        if tail == self.knots.len() - 1 {
            self.visited.insert(self.knots[tail]);
        }
    }

    fn is_touching(&self, head: usize, tail: usize) -> bool {
        self.is_overlap(head, tail) || self.is_touching_cardinal(head, tail) || self.is_touching_diag(head, tail)
    }

    fn is_overlap(&self, head: usize, tail: usize) -> bool {
        (self.knots[tail].x == self.knots[head].x && self.knots[tail].y == self.knots[head].y)
    }

    fn is_touching_cardinal(&self, head: usize, tail: usize) -> bool {
        (self.knots[tail].x - 1 == self.knots[head].x && self.knots[tail].y == self.knots[head].y)        // touching right
            || (self.knots[tail].x + 1 == self.knots[head].x && self.knots[tail].y == self.knots[head].y) // touching left
            || (self.knots[tail].y - 1 == self.knots[head].y && self.knots[tail].x == self.knots[head].x) // touching up
            || (self.knots[tail].y + 1 == self.knots[head].y && self.knots[tail].x == self.knots[head].x) // touching down
    }

    fn is_touching_diag(&self, head: usize, tail: usize) -> bool {
        (self.knots[tail].x - 1 == self.knots[head].x && self.knots[tail].y - 1 == self.knots[head].y)        // touching diag right, up
            || (self.knots[tail].x + 1 == self.knots[head].x && self.knots[tail].y - 1 == self.knots[head].y) // touching diag left, up
            || (self.knots[tail].x - 1 == self.knots[head].x && self.knots[tail].y + 1 == self.knots[head].y) // touching diag right, down
            || (self.knots[tail].x + 1 == self.knots[head].x && self.knots[tail].y + 1 == self.knots[head].y) // touching diag left, down
    }
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        let moves: Vec<Move> = input.lines().into_iter()
            .map(Move::from)
            .collect();

        let mut bridge = Bridge::default();
        for mve in moves {
            bridge.do_move(&mve);
        }

        println!("{:?}", bridge.visited.len())
    }
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        let moves: Vec<Move> = input.lines().into_iter()
            .map(Move::from)
            .collect();

        let mut bridge = Bridge {
            knots: Vec::from([Point::default(); 10]),
            visited: HashSet::from([Point::default()]),
        };
        for mve in moves {
            bridge.do_move(&mve);
        }

        println!("{:?}", bridge.visited.len())
    }
}

const INPUT: &str = include_str!("input.txt");
//const INPUT: &str = include_str!("example.txt");
//const INPUT: &str = include_str!("example2.txt");
//const INPUT: &str = include_str!("test.txt");

fn main() {
    print!("Part 1: ");
    Part1::solve(INPUT);

    print!("Part 2: ");
    Part2::solve(INPUT);
}
