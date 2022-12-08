type Trees = Vec<Vec<usize>>;

trait Puzzle {
    fn solve(input: &str);
}

struct Part1;

struct Part2;

// reads | [
// 123   |   [1, 2, 3],
// 321   |   [3, 2, 1]
// into  | ]

fn read_trees(input: &str) -> Trees {
    let trees: Trees = input.lines().into_iter()
        .map(|l| l.chars().into_iter()
            .map(|c| c.to_digit(10).expect("num") as usize)
            .collect()
        )
        .collect();

    trees
}

fn is_visible_above(trees: &Trees, i: usize, j: usize) -> bool {
    let mut ii = 1;
    let mut highest: usize = 0;

    while i >= ii {
        let c = trees[i - ii][j];
        if c > highest {
            highest = c;
        }
        ii += 1;
    }

    highest < trees[i][j]
}

fn viewing_above(trees: &Trees, i: usize, j: usize) -> usize {
    let mut ii = 1;

    while i >= ii && trees[i - ii][j] < trees[i][j] {
        ii += 1;
    }

    if ii - 1 == i {
        ii -= 1;
    }

    //println!("Above: {:?}", ii);

    ii
}

fn is_visible_below(trees: &Trees, i: usize, j: usize) -> bool {
    let mut ii = 1;
    let mut highest: usize = 0;

    while ii < (trees.len() - i) {
        let c = trees[i + ii][j];
        if c > highest {
            highest = c;
        }
        ii += 1;
    }

    highest < trees[i][j]
}

fn viewing_below(trees: &Trees, i: usize, j: usize) -> usize {
    let mut ii = 1;

    while ii < (trees.len() - i) && trees[i + ii][j] < trees[i][j] {
        ii += 1;
    }

    if ii == (trees.len() - i) {
        ii -= 1;
    }

   // println!("Below: {:?}", ii);
    ii
}

fn is_visible_left(trees: &Trees, i: usize, j: usize) -> bool {
    let mut jj = 1;
    let mut highest: usize = 0;

    while j >= jj {
        let c = trees[i][j - jj];
        if c > highest {
            highest = c;
        }
        jj += 1;
    }

    highest < trees[i][j]
}

fn viewing_left(trees: &Trees, i: usize, j: usize) -> usize {
    let mut jj = 1;

    while j >= jj && trees[i][j - jj] < trees[i][j] {
        jj += 1;
    }

    if jj - 1 == j {
        jj -= 1;
    }

    //println!("Left: {:?}", jj);

    jj
}

fn is_visible_right(trees: &Trees, i: usize, j: usize) -> bool {
    let mut jj = 1;
    let mut highest: usize = 0;

    while jj < (trees[i].len() - j) {
        let c = trees[i][j + jj];
        if c > highest {
            highest = c;
        }
        jj += 1;
    }

    highest < trees[i][j]
}

fn viewing_right(trees: &Trees, i: usize, j: usize) -> usize {
    let mut jj = 1;

    while jj < (trees[i].len() - j) && trees[i][j + jj] < trees[i][j] {
        jj += 1;
    }

    if jj == (trees[i].len() - j) {
        jj -= 1;
    }

    //println!("Right: {:?}", jj);

    jj
}


impl Puzzle for Part1 {
    fn solve(input: &str) {
        let trees = read_trees(input);
        let mut i: usize = 1;
        let mut j: usize;

        let mut count_visible: usize = 0;

        while i < trees.len() - 1 {
            j = 1;
            let row = &trees[i];

            while j < row.len() - 1 {
                let mut is_visible;

                is_visible = is_visible_above(&trees, i, j);
                is_visible = is_visible_below(&trees, i, j) || is_visible;
                is_visible = is_visible_left(&trees, i, j) || is_visible;
                is_visible = is_visible_right(&trees, i, j) || is_visible;

                if is_visible {
                    count_visible += 1;
                }

                j += 1;
            }

            i += 1;
        }

        let edge = (2 * trees.len()) + (2 * (trees[1].len() - 2));
        println!("e: {:?}, t: {:?}, s: {:?}", edge, count_visible, edge + count_visible)
    }
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        let trees = read_trees(input);
        let mut i: usize = 1;
        let mut j: usize;

        let mut best: usize = 0;

        while i < trees.len() - 1 {
            j = 1;
            let row = &trees[i];

            while j < row.len() - 1 {
                //println!("Check: i: {:?}, j: {:?}", i, j);

                let cur = viewing_above(&trees, i, j)
                    * viewing_below(&trees, i, j)
                    * viewing_left(&trees, i, j)
                    * viewing_right(&trees, i, j);

                if cur > best {
                    best = cur;
                }

                j += 1;
            }

            i += 1;
        }

        println!("{:?}", best);
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
