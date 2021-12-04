use advent_of_code::puzzle::Puzzle;

struct Part1;
struct Part2;

fn to_vec(inp: &str) -> Vec<usize> {
    inp.split("\n").map(|l| l.parse().unwrap()).collect()
}

fn first_offending(nums: &Vec<usize>, preamble: usize) -> usize {
    // [0, 1, 2, 3, 4, 5, 6, 7]
    //              p
    //                 i
    //  l
    //              r
    // - - - - - - - - - - - - 
    //                    i
    //    l            r
    for i in (preamble + 1)..nums.len() {
        let l = i - preamble;
        let r = i;

        let target = nums[i];

        let mut found = false;
        for j in l..r {
            for k in l..r {
                let nj = nums[j];
                let nk = nums[k];
                println!("nj: {}, nk: {}, s: {}, t: {}", nj, nk, nj + nk, target);
                if nj + nk == target {
                    println!("FOUND");
                    found = true;
                    break;
                }
            }
            
            if found {
                break
            }
        }

        if !found {
            return nums[i];
        }
    }

    return 0;
}

impl Puzzle for Part1 {

    fn solve(input: &str) {
        let nums = to_vec(input);
        //println!("{}", first_offending(&nums, 5))
        println!("{}", first_offending(&nums, 25))
    }
}

impl Puzzle for Part2 {

    fn solve(input: &str) {
        let nums = to_vec(input);
        let target = first_offending(&nums, 25);
        let mut sum = 0;

        let mut i = 0;
        let mut j = 1;
        let mut acc = Vec::new(); 
        let mut accs = acc.iter().sum::<usize>();

        while accs < target {
            acc.push(nums[i]);
            accs = acc.iter().sum::<usize>();
            i += 1;

            println!("l: {:?}, s: {}", acc, accs);

            if accs > target {
                acc.clear();
                accs = acc.iter().sum::<usize>();
                i = j;
                j += 1;
            }

            if accs == target {
                acc.sort();
                sum = acc.first().unwrap() + acc.last().unwrap();
            }
        }


        println!("{}", sum)
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
