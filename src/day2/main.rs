use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {

    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;

    let file = BufReader::new(File::open("./src/day2/movement.txt").expect("open failed"));

    for line in file.lines() {
        if let Ok(ip) = line {

            // split direction and increment
            let split = ip.split(" ");
            let vec: Vec<&str> = split.collect();
            let increment = vec[1].parse::<i32>().unwrap();

            // Matching pattern
            match vec[0] {
                "forward" => {
                    forward += increment;
                    depth += increment * aim;
                },
                // first part of the exercise
                // "up"=> depth -= increment,
                // "down" => depth += increment,

                // second part of the exercise 
                "up"=> aim -= increment, 
                "down" => aim += increment,
                _ => println!("no direction found")
            }
        }
        }   
        // convert to larger type
        let depth:i64 = i64::from(depth);
        let forward:i64 = i64::from(forward);
        let d_times_f:i64 = depth*forward;

        println!("Depth: {}, forward: {}, d*f= {}", depth.to_string(), forward.to_string(), d_times_f.to_string())

}

// // FIRST EXERCISE WITHOUT AIM
// fn main() {
//
//     let mut forward = 0;
//     let mut depth = 0; // max i32 number
//
//     // read in movement.txt containing numbers
//     if let Ok(lines) =  read_lines("./src/day2/movement.txt") {
//
//         // Consumes the iterator, returns an (Optional) String
//         for line in lines {
//             if let Ok(ip) = line {
//
//                 // split direction and increment
//                 let split = ip.split(" ");
//                 let vec: Vec<&str> = split.collect();
//                 let increment = vec[1].parse::<i32>().unwrap();
//
//                 // Matching pattern
//                 match vec[0] {
//                     "forward" => forward += increment,
//                     "up"=> depth -= increment,
//                     "down" => depth += increment,
//                     _ => println!("no direction found")
//                 }
//             }
//         }
//         let d_times_f = depth*forward;
//         println!("Depth: {}, forward: {}, d*f= {}", depth.to_string(), forward.to_string(), d_times_f.to_string())
//     }
// }
