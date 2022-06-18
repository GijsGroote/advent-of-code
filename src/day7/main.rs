use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let mut crab_positions:Vec<u32> = vec![Default::default()];


    // read in data
    let file = BufReader::new(File::open("./src/day7/crabs.txt").expect("open failed"));

    for line in file.lines() {
        if let Ok(line) = line {
            for char in line.split(',') {
               let position: u32 = char.parse::<u32>().unwrap();
               crab_positions.push(position); 
            }
        }
    }

    println!("could you print the crab positions for me?");
    
    println!("{crab_positions:?}");
   
    }

