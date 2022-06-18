use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let days:usize = 256;
    let mut fishes:Vec<u128> = vec![Default::default(); 9];


    // read in data
    let file = BufReader::new(File::open("./src/day6/ages.txt").expect("open failed"));

    for line in file.lines() {
        if let Ok(line) = line {
            for char in line.split(',') {
                let timer: usize = char.parse::<usize>().unwrap();
                fishes[timer] += 1
            }
        }
    }

    // simulate the fishes for days
    simulate(days, &mut fishes);

    // count the fishes
    let mut total_fish: u128 = 0;
    for fish in fishes {
        total_fish += fish;
    }

    println!("total number of fish {total_fish}");

}


fn simulate(days: usize, fishes: &mut Vec<u128>){
    // simulate the growth of fishes for a number of days

     // base case
    if days <= 0 {
        return;
    }

    let day0 = fishes[0];

    // println!("day: {}", days);
    // println!("Fishes: {fishes:?}");

    let mut day_counter:usize = 0;

    loop {
        if day_counter >= 8 {break;}

        fishes[day_counter] = fishes[day_counter + 1];

        day_counter += 1;
    }

    // add fishes to day 5 and day 7 because of the newborn
    fishes[6] += day0;
    fishes[8] = day0;

    return simulate(days-1, fishes);
}
