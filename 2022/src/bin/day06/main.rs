use std::default::Default;

trait Puzzle {
    fn solve(input: &str);
}

struct Part1;

struct Part2;

const REG_SIZE: usize = 14;

#[derive(Debug)]
struct Reg {
    found: [char; REG_SIZE],
    insert_count: usize,
}

impl Default for Reg {
    fn default() -> Self {
        Reg {
            found: ['-'; REG_SIZE],
            insert_count: 0,
        }
    }
}

impl Reg {
    fn push(&mut self, el: char) {
        if self.insert_count < REG_SIZE {
            self.found[self.insert_count] = el;
        } else {
            // shift the array when 'full'
            for i in 1..self.found.len() {
                self.found[i - 1] = self.found[i];
            }

            self.found[REG_SIZE - 1] = el;
        }

        self.insert_count += 1;
    }

    fn is_full(&self) -> bool {
        self.insert_count >= REG_SIZE
    }

    fn has_duplicate(&self) -> bool {
        let mut is_duplicate = false;

        let mut i: usize = 0;
        let mut j: usize = 1;

        while i < self.found.len() && !is_duplicate {
            while j < self.found.len() && !is_duplicate {
                if i != j {
                    is_duplicate = self.found[i] == self.found[j];
                }
                j += 1;
            }
            j = 1;
            i += 1;
        }

        is_duplicate
    }
}

impl Puzzle for Part1 {
    fn solve(input: &str) {
        let mut reg = Reg::default();
        let mut no_repeat: usize = 0;
        let mut i: usize = 0;

        let char_arr: Vec<char> = input.chars().collect();

        while no_repeat == 0 && i < input.len() {
            reg.push(char_arr[i]);
            if reg.is_full() && !reg.has_duplicate() {
                no_repeat = i
            }
            i += 1;
        }

        println!("{:?}", no_repeat + 1)
    }
}

impl Puzzle for Part2 {
    fn solve(input: &str) {
        println!("NOT STARTED")
    }
}

const INPUT: &str = include_str!("input.txt");
//const INPUT: &str = include_str!("example.txt");

fn main() {
    print!("Part 1: ");
    Part1::solve(INPUT);

    print!("Part 2: ");
    Part1::solve(INPUT);
    //Part2::solve(INPUT);
}

#[cfg(test)]
mod test {
    use crate::Reg;

    #[test]
    fn dupl() {
        let mut reg = Reg {
            found: ['a', 'b', 'c', 'a'],
            insert_count: 0
        };

        assert!(reg.has_duplicate());

        reg.found = ['a', 'b', 'c', 'd'];

        assert!(!reg.has_duplicate())
    }
}