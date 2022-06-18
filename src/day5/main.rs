use std::cmp::{min, max};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // finds most dangurous hydrothermal vents

    let lines: Vec<Vec<u8>> = read_input("./src/day5/lines.txt", false);

    println!("number of dangerous points: {} \n(only counting vertical and horizontal lines)",
             count_overlapping_points(&lines));

    let lines: Vec<Vec<u8>> = read_input("./src/day5/lines.txt", true);

    println!("number of dangerous points: {} \n(now also counting diagonal lines)",
             count_overlapping_points(&lines));

    // print_lines(&lines);
}

fn count_overlapping_points(lines: &Vec<Vec<u8>>) -> i32 {
    // counts how many points overlap, they are larger or equal then 2 in matrix lines
    let mut counter: i32 = 0;

    for line in lines {
        for number in line {
            if *number >= 2 {
                counter += 1;
            }
        }
    }
    counter
}

fn print_lines(lines: &Vec<Vec<u8>>) {
    for line in lines {
        for number in line {
            if *number == 0 {
                print!(" . ");
            } else {
                print!(" {} ", number);
            }
        }
        println!();
    }
}

fn read_input(path: &str, count_diagonal: bool) -> Vec<Vec<u8>> {
    // reads the horizontal and vertical lines in example_lines.txt
    // returns a variable which is x and y position in lines lines[y_pos][x_pos]

    let file = BufReader::new(File::open(path).expect("open failed"));

    // find largest and smallest x and y
    let (mut smallest_x, mut smallest_y, mut largest_x, mut largest_y): (usize, usize, usize, usize) = (100, 100, 0, 0);
    let (mut x1, mut y1, mut x2, mut y2): (usize, usize, usize, usize);
    for line in file.lines() {
        if let Ok(line) = line {
            let points: Vec<&str> = line.split(|c| c == ',' || c == ' ').collect();
            x1 = points[0].parse::<usize>().unwrap();
            y1 = points[1].parse::<usize>().unwrap();
            x2 = points[3].parse::<usize>().unwrap();
            y2 = points[4].parse::<usize>().unwrap();

            // find smallest x and y
            if x1 < smallest_x { smallest_x = x1; }
            if x2 < smallest_x { smallest_x = x2; }
            if y1 < smallest_y { smallest_y = y1; }
            if y2 < smallest_y { smallest_y = y2; }

            // find largest x and y
            if x1 > largest_x { largest_x = x1; }
            if x2 > largest_x { largest_x = x2; }
            if y1 > largest_y { largest_y = y1; }
            if y2 > largest_y { largest_y = y2; }
        }
    }

    // create and fill lines matrix
    let mut lines: Vec<Vec<u8>> = vec![vec![Default::default(); largest_x + 1]; largest_y + 1];

    let file = BufReader::new(File::open(path).expect("open failed"));
    for line in file.lines() {
        if let Ok(line) = line {
            let points: Vec<&str> = line.split(|c| c == ',' || c == ' ').collect();
            x1 = points[0].parse::<usize>().unwrap();
            y1 = points[1].parse::<usize>().unwrap();
            x2 = points[3].parse::<usize>().unwrap();
            y2 = points[4].parse::<usize>().unwrap();

            let xmin = min(x1, x2);
            let ymin = min(y1, y2);
            let xmax = max(x1, x2);
            let ymax = max(y1, y2);

            if x1 == x2 {
                for i in ymin..(ymax + 1) {
                    lines[i][x1] += 1;
                }
            } else if y1 == y2 {
                for i in xmin..(xmax + 1) {
                    lines[y1][i] += 1;
                }
            } else if count_diagonal {
                let dydx = (y1 as isize - y2 as isize) / (x1 as isize - x2 as isize);
                for i in 0..(xmax - xmin + 1) {
                    if dydx == 1 {
                        lines[ymin + i][xmin + i] += 1;
                    } else if dydx == -1 {
                        lines[ymax - i][xmin + i] += 1;
                    } else { println!("going nowhere, error"); }
                }
            }
        }
    }
    lines
}
