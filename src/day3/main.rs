use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let base_2: i32 = 2;
    //  read in data
    let file = BufReader::new(File::open("./src/day3/diagnostic_report.txt").expect("open failed"));

    // find number of rows and columns
    let (n_rows, n_columns) = count_rows_and_columns();
    println!("diagnostic report contains {} columns, and {} rows", n_columns, n_rows);


    let mut diagnostic: Vec<Vec<u8>> = vec![vec![Default::default(); n_columns]; n_rows];
    let mut diag_trans: Vec<Vec<u8>> =vec![vec![Default::default(); n_rows]; n_columns];

    // fill diagnostic report with data
    for (row_index, line) in file.lines().enumerate() {
        for (column_index, char) in line.expect("lines failed").chars().enumerate() {
            diagnostic[row_index][column_index] = char as u8 - '0' as u8;
            diag_trans[column_index][row_index] = char as u8 - '0' as u8;
        }
    }

    // THIS IS THE FIRST PART
    // count most and least common occurrence per column
    let (_gamma_binary, _epsilon_binary) = count_occurrence_per_column(&diagnostic, n_columns);

    // create two boolean vectors
    let mut o2_left: Vec<bool> = vec![true; n_rows];
    let mut co2_left: Vec<bool> = vec![true; n_rows];
    let mut most_common:u8;
    let mut o2_found_i = 0;
    let mut co2_found_i = 0;
    let mut true_left:i32;

    // loop through columns
    for row in diag_trans.iter() {

        // oxygen
        most_common = find_most_common_for_row(&o2_left, &row);
        // number of 1s equals number of 0s -> take a 1
        if most_common == 2 {most_common = 1; }
        // take away all the non-commons and count
        true_left = 0;
        for (j, b) in row.iter().enumerate() {
            if *b != most_common {
                o2_left[j] = false
            }
            if o2_left[j] == true {
                true_left += 1;
            }
        }

        // print!("o2 left: {}", true_left);
        if true_left == 1 {
            println!("found last o2");
            for (i, bool) in o2_left.iter().enumerate() {
                if *bool == true {
                    println!("found indices for o2, this should only print once");
                    o2_found_i = i;
                }
            }
        }
        // co2
        most_common = find_most_common_for_row(&co2_left, &row);

        // take the least common instead of most common. most_common actually is least common here
        if most_common == 0 {
            most_common = 1;
        }
        else if most_common == 1 {
            most_common = 0
        }
        else if most_common == 2 {
            most_common = 0;
        }

        let mut true_left = 0;
        // take away all the non-commons and count
        for (j, b) in row.iter().enumerate() {
            if *b != most_common {
                co2_left[j] = false
            }
            if co2_left[j] == true {
                true_left += 1;
            }
        }
        println!(" co2 left: {}", true_left);

        if true_left == 1 {
            println!("found last co2");
            for (i, bool) in co2_left.iter().enumerate() {
                if *bool == true {
                    co2_found_i = i;
                }
            }
        }
    }

    println!("o2 indices {}, co2 indices {}", o2_found_i, co2_found_i);
    for element in diagnostic[co2_found_i].iter() {
        println!("hex values: {}", element);
    }

    let mut o2_dec = 0;
    let mut co2_dec = 0;

    for (i, num) in diagnostic[o2_found_i].iter().enumerate() {
        if *num == 1 {
            o2_dec += base_2.pow((n_columns -1 - i) as u32);
        }
    }
    for (i, num) in diagnostic[co2_found_i].iter().enumerate() {
        if *num == 1 {
            co2_dec += base_2.pow((n_columns -1 - i) as u32);
        }
    }
    println!("o2_dec: {}. co2_dec {}", o2_dec, co2_dec);

    let answer = o2_dec * co2_dec;
    println!("answer {}", answer);

}

fn count_rows_and_columns() -> (usize, usize) {
    // count the number columns (characters in the first row)
    // and count the number of rows in the text file specified by path

    let file = BufReader::new(File::open("./src/day3/diagnostic_report.txt").expect("open failed"));

    // keep track if the number of rows already was counted
    let mut should_count_columns: bool = true;

    let mut n_columns = 0;
    let mut n_rows = 0;

    for line in file.lines() {
        n_rows += 1;

        if should_count_columns {
            for _ in line.expect("lines failed").chars() {
                n_columns += 1;
            }
            should_count_columns = false;
        }
    }
    (n_rows, n_columns)
}

fn count_occurrence_per_column(diagnostic_report: &Vec<Vec<u8>>, n_columns: usize) -> (Vec<i32>, Vec<i32>) {
    // find the highest occurrence per column

    // create 2D array
    let mut count_binary: Vec<Vec<i32>> = vec![vec![Default::default(); n_columns]; 2];

    // Count 1's and 0's for every column
    for row in diagnostic_report.iter() {
        // Count 1's and 0's in every row
        for (i, char) in row.iter().enumerate() {
            if *char == 1 {
                count_binary[1][i] += 1;
            } else if *char == 0 {
                count_binary[0][i] += 1;
            } else { println!("Error, u8 is not a 1 or a 0, it's a: {}", *char) }

        }
    }

    let mut gamma_binary: Vec<i32> = vec![Default::default(); n_columns as usize];
    let mut epsilon_binary: Vec<i32> = vec![Default::default(); n_columns as usize];
    let mut gamma_decimal = 0;
    let mut epsilon_decimal = 0;
    let mut counter: usize = 0;
    let base_2: i32 = 2;

    loop {
        // stopping the loop
        if counter == n_columns as usize {
            break;
        }
        // find the most common, the 1 or the 0
        if count_binary[0][counter] < count_binary[1][counter] {
            gamma_binary[counter] = 1;
            epsilon_binary[counter] = 0;
        } else if count_binary[0][counter] > count_binary[1][counter] {
            gamma_binary[counter] = 0;
            epsilon_binary[counter] = 1;
        } else {
            println!("warning no common bit: {} zeroes and {} ones", count_binary[0][counter], count_binary[1][counter])
        }

        // Convert binary to decimal number
        if gamma_binary[counter] == 1 && epsilon_binary[counter] == 0 {
            gamma_decimal += base_2.pow((n_columns -1 - counter) as u32);
        } else if gamma_binary[counter] == 0 && epsilon_binary[counter] == 1 {
            epsilon_decimal += base_2.pow((n_columns -1 - counter) as u32);
        } else {
            println!("error gamma: {} and epsilon: {} are equal", gamma_binary[counter], epsilon_binary[counter])
        };

        counter += 1;
    };
    println!("Gamma binary: {:?}, Epsilon binary: {:?}", gamma_binary, epsilon_binary);
    println!("Gamma decimal: {}, Epsilon decimal: {:?}", gamma_decimal, epsilon_decimal);
    println!("Power = Gamma * Epsilon = {}", gamma_decimal * epsilon_decimal);

    return (gamma_binary, epsilon_binary);
}

fn find_most_common_for_row(is_left: &Vec<bool>, row: &Vec<u8>) -> u8 {
    // calculate for most common number in column_i, 0 or 1 in rows for which is_left corresponds with true

    let mut n_zeros = 0;
    let mut n_ones = 0;

    for (i, number) in row.iter().enumerate() {
        if is_left[i] == true {
            if *number == 1 {
                n_ones += 1;
            } else if *number == 0 {
                n_zeros += 1;
            } else {
                println!("Warning: {} is not a 1 or a 0", *number);
            }
        }
    }

    return if n_zeros > n_ones {
        0
    } else if n_ones > n_zeros {
        1
    } else if n_zeros == n_ones {
        2
    } else {
        println!("Warning, n_ones: {} is not comparable with n_zeros: {}", n_ones, n_zeros);
        2
    };
}
