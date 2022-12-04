
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

                let (l, r) = row.split_at(row.len() / 2);
                let mut left = Vec::from(l);
                let mut right = Vec::from(r);
                left.sort();
                right.sort();

                let mut res :usize = 0;
                let mut i :usize = 0;
                let mut j :usize = 0;

                while i < left.len() && j < right.len() && res == 0 {

                    if left[i] == right[j] {
                        res = left[i]
                    }

                    if left[i] < right[j] {
                        i += 1;
                    }

                    if left[i] > right[j] {
                        j += 1;
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
            .map(|mut v| {
                v.sort();
                v
            })
            .collect();

        let mut sum :usize = 0;
        let mut i :usize = 0;

        while i < numbers.len()  {

            let first = &numbers[i];
            let second = &numbers[i + 1];
            let third = &numbers[i + 2];

            let mut found :bool = false;
            let mut j :usize = 0;
            let mut k :usize = 0;
            let mut l :usize = 0;

            while j < first.len() && k < second.len() && l < third.len() && !found {
                //println!("R: {:?}, j: {:?}, k: {:?}, l: {:?}", i, j, k , l);
                //println!("f: {:?}, s: {:?}, t: {:?}", first[j], second[k], third[l]);

                if first[j] < second[k] {
                    j += 1;
                }

                if first[j] > second[k] {
                    k += 1;
                }

                if first[j] == second[k] {
                    while third[l] < first[j] && l < third.len() {
                        l += 1;
                    }

                    if first[j] == third[l] {
                        sum += first[j];
                        found = true;
                    } else {
                        // current match in first and second not found in third, advance
                        j += 1;
                    }
                }
            }

            i += 3;
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
