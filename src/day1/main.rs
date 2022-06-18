use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    // Calculates the number of increases from numbers in a text file
    // Then, calculate the number of increases from the sum sliding window
    // read in depths.txt containing numbers
    let file = BufReader::new(File::open("./src/day1/depths.txt").expect("open failed"));
    let mut numbers: Vec<u32> = Vec::new();

    for line in file.lines() {
       if let Ok(line) = line {
           numbers.push(line.parse::<u32>().unwrap());      
       }
    }
    
    // println!("The numbers: {numbers:?}");
    let increase: u32 = count_number_of_increases(&numbers);
    
    println!("the number of increases was: {}", increase.to_string());
    println!("Now the average is calculated over a window of 3 samples");

    
    // average over 3 numbers
    let mut numbers_sum: Vec<u32> = vec![0; numbers.len() - 2];

    for (i, number) in numbers.iter().enumerate() {
        if i >= numbers.len()-2 {
            continue;
        }
        numbers_sum[i] = number + numbers[i+1] + numbers[i+2];
        //println!("some average numbers: {numbers_average:?}");
    }
    let increase_sum: u32 = count_number_of_increases(&numbers_sum);
    println!("the number of increases with the sliding window was: {}", increase_sum.to_string());

}

fn count_number_of_increases(numbers: &Vec<u32>) -> u32 {
    // Count the number of increases in a vector 
    let mut number_old = 2147483647; // max i32 number
    let mut increase = 0;

    for number in numbers {
        if *number > number_old {
            increase += 1;
        }
        number_old = *number;
    }
// return increase 
increase
} 

