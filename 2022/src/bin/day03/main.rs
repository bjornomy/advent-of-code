use std::iter::repeat_with;

trait Puzzle {
    fn solve(input: &str);
}
struct Part1;
struct Part2;

fn map_to_numbers(row :&str) -> Vec<usize> {
    row.chars()
        .map(|c| {
            let ascii = c as usize;
            if ascii > 96 { // assume lower
                ascii - 96
            } else {        // should be upper
                ascii - 38
            }
        })
        .collect()
}

impl Puzzle for Part1 {

    fn solve(input: &str) {

        let sum :usize = input.split('\n').collect::<Vec<&str>>().into_iter()
            .map(map_to_numbers)
            .map(|row| {

                let mut left = &row[0..(row.len() / 2)];
                let mut right = &row[(row.len() / 2)..];

                let mut res :usize = 0;

                for i in 0..left.len() {
                    for j in 0..right.len() {
                        if left[i] == right[j] {
                            res = left[i]
                        }
                    }
                }

                res
            })
            .sum();

        println!("{:?}", sum)
    }
}

impl Puzzle for Part2 {

    fn solve(input: &str) {
        let numbers :Vec<Vec<usize>> = input.split('\n').collect::<Vec<&str>>().into_iter()
            .map(map_to_numbers)
            .collect();

        let mut sum :usize = 0;

        for i in 0..numbers.len() {
            if (i + 1) % 3 == 0 {

                let first = &numbers[i - 2];
                let second = &numbers[i - 1];
                let third = &numbers[i];

                let mut found :bool = false;

                for j in 0..first.len() {
                    for k in 0..second.len() {
                        if first[j] == second[k] { // if first and second are eq, we can check in third
                            for l in 0..third.len() {
                                if first[j] == third[l] {
                                    sum += first[j];
                                    found = true;
                                    break
                                }
                            }
                        }

                        if found {
                            break
                        }
                    }

                    if found {
                        break
                    }
                }
            } else {
                // only process every 3 rows
                continue
            }
        }

        println!("{:?}", sum)
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
