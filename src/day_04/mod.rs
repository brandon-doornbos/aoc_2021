use std::{fs::File, io::Read};

const BOARD_SZ: usize = 5;
const BOARD_AMT: usize = 100;
type Board = [[usize; BOARD_SZ]; BOARD_SZ];
type BoardsContainer = [Board; BOARD_AMT];

pub fn main() {
    let mut input_file = File::open("./src/day_04/input.txt").unwrap();
    let mut input_raw = String::new();
    input_file.read_to_string(&mut input_raw).unwrap();

    let mut input = input_raw.trim().split('\n');
    let calls: Vec<usize> = input
        .next()
        .unwrap()
        .split(',')
        .map(|str| str.parse().unwrap())
        .collect();
    input.next();

    let mut boards: BoardsContainer = [[[0; BOARD_SZ]; BOARD_SZ]; BOARD_AMT];
    let mut board = 0;
    let mut lines = 0;
    for line in input {
        if line.is_empty() {
            board += 1;
            lines = 0;
            continue;
        }
        for (i, num) in line.trim().split_whitespace().enumerate() {
            boards[board][lines][i] = num.parse().unwrap();
        }
        lines += 1;
    }

    println!("Part 1: {}", part_1(&boards, &calls));
    println!("Part 2: {}", part_2(&boards, &calls));
}

fn part_1(input: &BoardsContainer, calls: &[usize]) -> usize {
    let mut checks: BoardsContainer = [[[0; BOARD_SZ]; BOARD_SZ]; BOARD_AMT];

    for call in calls {
        for board in 0..BOARD_AMT {
            for row in 0..BOARD_SZ {
                for col in 0..BOARD_SZ {
                    if input[board][row][col] == *call {
                        checks[board][row][col] = 1;
                    }
                }
            }
            if check_board(&checks[board]) {
                let mut sum: usize = 0;

                for row in 0..BOARD_SZ {
                    for col in 0..BOARD_SZ {
                        if checks[board][row][col] == 0 {
                            sum += input[board][row][col];
                        }
                    }
                }

                return sum * call;
            }
        }
    }

    panic!();
}

fn part_2(input: &BoardsContainer, calls: &[usize]) -> usize {
    let mut checks: BoardsContainer = [[[0; BOARD_SZ]; BOARD_SZ]; BOARD_AMT];
    let mut boards: [bool; BOARD_AMT] = [false; BOARD_AMT];

    for call in calls {
        for board in 0..BOARD_AMT {
            for row in 0..BOARD_SZ {
                for col in 0..BOARD_SZ {
                    if input[board][row][col] == *call {
                        checks[board][row][col] = 1;
                    }
                }
            }
            if check_board(&checks[board]) {
                boards[board] = true;

                if !boards.iter().any(|board| !board) {
                    let mut sum: usize = 0;

                    for row in 0..BOARD_SZ {
                        for col in 0..BOARD_SZ {
                            if checks[board][row][col] == 0 {
                                sum += input[board][row][col];
                            }
                        }
                    }

                    return sum * call;
                }
            }
        }
    }

    panic!();
}

#[derive(Clone, Copy)]
struct Line {
    row: usize,
    col: usize,
}
type Check = [Line; BOARD_SZ];

fn check_board(board: &Board) -> bool {
    let mut check: Check = [Line { row: 0, col: 0 }; BOARD_SZ];

    for row in 0..BOARD_SZ {
        for col in 0..BOARD_SZ {
            if board[row][col] == 1 {
                check[row].row += 1;
                check[col].col += 1;
            }
        }
    }

    for line in check {
        if line.row == BOARD_SZ || line.col == BOARD_SZ {
            return true;
        }
    }

    false
}
