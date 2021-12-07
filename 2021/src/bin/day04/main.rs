use advent_of_code_2021::puzzle::Puzzle;

struct Part1;
struct Part2;

#[derive(Debug)]
struct Board {
    numbers: Vec<Vec<usize>>,
    marked: Vec<Vec<bool>>,
    bingo: bool
}

impl Board {

    fn from_lines(lines: Vec<&str>) -> Self {
        let mut board = Board {
            numbers: Vec::new(),
            marked: Vec::new(),
            bingo: false
        };

        for l in lines {
            let mut row = Vec::new();
            let mut marked_row = Vec::new();

            l.split(' ').into_iter()
                .filter(|&el| el != " " && el != "")
                .for_each(|num| {
                    row.push(num.parse::<usize>().unwrap());
                    marked_row.push(false);
                });

            board.numbers.push(row);
            board.marked.push(marked_row);
        }

        // println!("{:?}", board);

        board
    }

    fn is_bingo(&mut self) -> bool {
        if self.bingo {
            return self.bingo;
        }

        let row_bingo = self.marked.clone().into_iter().any(|row| {
            row.into_iter().reduce(|l, r| l && r).unwrap()
        });

        let mut column_bingo = false;

        for i in 0..self.marked.len() {
            let row = &self.marked[i];
            column_bingo = true;

            for j in 0..row.len() {
                column_bingo &= self.marked[j][i];
            }
        }

        self.bingo = row_bingo || column_bingo;

        return self.bingo;
    }

    fn mark(&mut self, number: usize) -> bool {
        for i in 0..self.numbers.len() {
            let row = &self.numbers[i];

            for j in 0..row.len() {
                if row[j] == number {
                    // println!("{} == {}", row[j], number);
                    self.marked[i][j] = true;
                    return true
                }
            }
        }

        return false
    }

    fn total_unmarked(&self) -> usize {
        let mut result :usize = 0;
        for i in 0..self.marked.len() {
            let row = &self.marked[i];

            for j in 0..row.len() {
                if !row[j] {
                    result += self.numbers[i][j];
                }
            }
        }

        result
    }
}

impl Puzzle for Part1 {

    fn solve(input: &str) {
        let mut numbers_and_boards = input.split("\n\n").collect::<Vec<&str>>();
        let numbers :Vec<usize> = numbers_and_boards.remove(0).split(',').into_iter()
            .map(|elem| elem.parse::<usize>().unwrap())
            .collect();

        let mut boards :Vec<Board> = numbers_and_boards.into_iter()
            .map(|lines| Board::from_lines(lines.split('\n').collect::<Vec<&str>>()))
            .collect();

        let mut winning_board :usize = 0;
        let mut last_num :usize = numbers[0];
        for num in numbers {
            // println!("Drawing {}", num);
            for i in 0..boards.len() {

                let marked = boards[i].mark(num);
                // if marked {
                //     println!("Marked {}", i);
                // }

                if marked && boards[i].is_bingo() {
                    // println!("WINNER {}", i);
                    winning_board = i;
                    last_num = num;
                    break;
                }
            }

            if winning_board != 0 {
                break;
            }
        }

        // println!("{:?}", boards[winning_board]);
        println!("{}", boards[winning_board].total_unmarked() * last_num)
    }
}

impl Puzzle for Part2 {

    fn solve(input: &str) {
        let mut numbers_and_boards = input.split("\n\n").collect::<Vec<&str>>();
        let numbers :Vec<usize> = numbers_and_boards.remove(0).split(',').into_iter()
            .map(|elem| elem.parse::<usize>().unwrap())
            .collect();

        let mut boards :Vec<Board> = numbers_and_boards.into_iter()
            .map(|lines| Board::from_lines(lines.split('\n').collect::<Vec<&str>>()))
            .collect();

        let mut num_boards_to_win :usize = 0;
        let mut last_board :usize = 0;
        let mut last_num :usize = numbers[0];
        for num in numbers {
            // println!("Drawing {}", num);
            for i in 0..boards.len() {

                if !boards[i].is_bingo() {
                    let marked = boards[i].mark(num);
                    // if marked {
                    //     println!("Marked {}", i);
                    // }

                    if marked && boards[i].is_bingo() {
                        // println!("WINNER {}", i);
                        num_boards_to_win += 1;
                    }
                }


                last_num = num;

                if num_boards_to_win == boards.len() - 1 {
                    last_board = i;
                    break;
                }
            }

            if last_board != 0 {
                break;
            }
        }

        // println!("{:?}", boards[winning_board]);
        println!("{}", boards[last_board].total_unmarked() * last_num)
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
