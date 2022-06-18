use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // bingo cheat sheet if you have the random numbers which will come

    let (n_rows_or_cols, random_numbers, bingo_boards) = read_bingo_data();
    let mut board_won_bool: Vec<bool> = vec![false; bingo_boards.len()];
    let mut bingo_bools: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; n_rows_or_cols]; n_rows_or_cols]; bingo_boards.len()];
    let mut i_last_bingo = 100000;

    // loop through all numbers
    for rand_num in random_numbers {
        let mut new_bingo = false;

        // check number in boards row's and columns
        for (i_board, board) in bingo_boards.iter().enumerate() {
            for (i_column, column) in board.iter().enumerate() {
                for (i_row, number) in column.iter().enumerate() {
                    if *number == rand_num {
                        bingo_bools[i_board][i_column][i_row] = true;
                    }
                }
            }
        }

        // check for bingo!
        for (i_board, board_bool) in bingo_bools.iter().enumerate() {
            let mut column_checker: Vec<bool> = vec![true; n_rows_or_cols];
            let mut row_checker: Vec<bool> = vec![true; n_rows_or_cols];

            // check bingo in every board
            if board_won_bool[i_board] == false {
                for (i_column, column) in board_bool.iter().enumerate() {
                    // check rows and columns
                    for (i_row, row) in column.iter().enumerate() {
                        if *row == false {
                            // found a unchecked number, no bingo in this column or row
                            column_checker[i_column] = false;
                            row_checker[i_row] = false;
                        }
                    }
                }

                for column_bool in column_checker.iter() {
                    if *column_bool {
                        board_won_bool[i_board] = true;
                        println!("Bingo at board: {}", i_board);
                        new_bingo = true;
                        i_last_bingo = i_board;
                        // print_board(&bingo_boards[i_board], &bingo_bools[i_board]);
                        // calculate_score(rand_num, &bingo_bools[i_board], &bingo_boards[i_board]);
                        // return;
                    }
                }

                for row_bool in row_checker.iter() {
                    if *row_bool {
                        board_won_bool[i_board] = true;
                        println!("Bingo at board: {}", i_board);
                        new_bingo = true;
                        i_last_bingo = i_board;

                    }
                }
            }
        }

        if new_bingo {
            // Count how many boards are left without bingo
            let mut n_bingos = 0;
            for board_bool in board_won_bool.iter() {
                if *board_bool {
                    n_bingos += 1;
                }
            }

            // Find absolute legend, first bingo of a board
            if n_bingos == 1 {
                println!("first bingo on board: {}", i_last_bingo);
                calculate_score(rand_num, &bingo_bools[i_last_bingo], &bingo_boards[i_last_bingo]);
                print_board(&bingo_boards[i_last_bingo], &bingo_bools[i_last_bingo]);
            }
            // Find absolute failure, last bingo of a board
            if n_bingos == bingo_boards.len() {
                println!("very last bingo on board: {}", i_last_bingo);
                calculate_score(rand_num, &bingo_bools[i_last_bingo], &bingo_boards[i_last_bingo]);
                print_board(&bingo_boards[i_last_bingo], &bingo_bools[i_last_bingo]);
            }
        }
    }
}

fn read_bingo_data() -> (usize, Vec<u8>, Vec<Vec<Vec<u8>>>) {
    // read in all bingo data en returning it in vector format

    // read in bingo data
    let file = BufReader::new(File::open("./src/day4/bingo.txt").expect("open failed"));

    // initiate variables
    let mut random_numbers: Vec<u8> = Default::default();
    let mut bingo_boards: Vec<Vec<Vec<u8>>> = Default::default();
    let mut board: Vec<Vec<u8>> = Default::default();
    let mut board_row_counter: i32 = 0;
    let mut n_rows_or_cols: usize = 0;

    for (i, line) in file.lines().enumerate() {
        match i {
            0 => {
                // initiate vector
                random_numbers = vec![Default::default(); line.as_ref().unwrap().split(",").count()];
                // initialize vector
                for (j, number) in line.unwrap().split(",").enumerate() {
                    let number = number.parse::<u8>().unwrap();
                    random_numbers[j] = number;
                }
            }
            1 => { continue; }
            _ => {
                if i == 2 {
                    n_rows_or_cols = line.as_ref().unwrap().split_whitespace().count();

                    // assuming the number of rows and columns in bingo are equal
                    board = vec![vec![Default::default(); n_rows_or_cols]; n_rows_or_cols];
                }

                if let Ok(line) = line {
                    if line.is_empty() {
                        continue;
                    } else {
                        for (j, number) in line.split_whitespace().enumerate() {
                            let number = number.parse::<u8>().unwrap();

                            board[board_row_counter as usize][j] = number;
                        }
                        board_row_counter += 1;
                    }
                    if board_row_counter == 5 {
                        let board_old = board;
                        bingo_boards.push(board_old);
                        board = vec![vec![Default::default(); n_rows_or_cols]; n_rows_or_cols];
                        board_row_counter = 0;
                    }
                }
            }
        }
    }
    (n_rows_or_cols, random_numbers, bingo_boards)
}

fn calculate_score(last_number: u8, bingo_bools: &Vec<Vec<bool>>, board: &Vec<Vec<u8>>) -> i32 {
    let mut sum: i32 = 0;
    for (i_column, column) in bingo_bools.iter().enumerate() {
        for (i_row, row_b) in column.iter().enumerate() {
            if !row_b {
                sum += board[i_column][i_row] as i32;
            }
        }
    }
    println!("sum of unchecked numbers is: {}, last number drawn was: {}", sum, last_number);
    let answer = sum * last_number as i32;
    println!("answer is: {}", answer);
    answer
}

fn print_board(board: &Vec<Vec<u8>>, board_bools: &Vec<Vec<bool>>) {
    for (i_column, column) in board.iter().enumerate() {
        for (i_row, _number) in column.iter().enumerate() {
            if board_bools[i_column][i_row] {
                print!("{}. ", board[i_column][i_row]);
            } else {
                print!("{} ", board[i_column][i_row]);
            }
            if i_row == 4 {
                println!(" ");
            }
        }
    }
    println!(" ");
}
