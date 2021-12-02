mod utils;

use std::{env, error::Error};
use utils::read_line_delimited;


fn main() -> Result<(), Box<dyn Error + 'static>>{
    
    let args = env::args().collect::<Vec<String>>();
    let problem: u32 = args[1].parse().map_err(|err| format!("{}: {}", err, args[1]))?;
    
    match problem {
        1 => {
            let mut increases = count_windowed_increases(1, "p1.txt");
            println!("Found {} increases", increases);

            increases = count_windowed_increases(3, "p1.txt");
            println!("Found {} Summed increases", increases)
        }
        2 => {
            let mut product = find_current_position(false);
            println!("Found product of distance and depth: {}", product);

            product = find_current_position(true);
            println!("Found product of distance and depth with aim: {}", product)
        }
        _ => println!("Problem {} not implemnted yet", problem)
    };

    Ok(())
}


// Count the number of "increases" between windows of given size for the input file
//
//  Args:
//      window_size: The size of the windows to compare     
//      file_name: The name of the file to read input from
//  Returns:
//      A count for the number of increases between windows of the given size
//
//  Example:
//      with a window size of 3
// 2
// |
// 3 -- | window1, sum = 3 + 4 + 5 = 12
// |    | 
// 4 -- | -- | window2, sum = 4 + 5 + 1 = 10
// |    |    |
// 5 -- |    |
// |         |
// 1 -- - -- |
//
// 10 < 12 so no increase
//
fn count_windowed_increases(window_size: usize, file_name: &str) -> u32 {
    let inputs = read_line_delimited(file_name);

    inputs
        .into_iter()
        .map(|val| val.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .windows(window_size)
        .map(|group| group.into_iter().fold(0, |acc, val| acc + val))
        .collect::<Vec<u32>>()
        .windows(2)
        .map(| window | window.to_vec())
        .collect::<Vec<Vec<u32>>>()
        .into_iter()
        .map(|window| {
            window[0] < window[1]
        })
        .fold(0, |acc, val| {
            match val {
                true => acc + 1,
                _ => acc
            }
        })
        
}
// Find the product of the distance and deph given the
// input file
//
// Args:
//      use_aim: A boolean indicating whether "aim" should be used
//
// Returns:
//      The product of the distance traveled and the depth reached by the submarine
//
fn find_current_position(use_aim: bool) -> u64 {
    let inputs = read_line_delimited("p2.txt");
    
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;
    
    for line in inputs {
        let parts: Vec<_> = line.split_whitespace().collect();
        let value = parts[1].parse::<u64>().unwrap();
        let direction = parts[0];

        match direction {
            "forward" => {
                if use_aim {
                    depth += aim * value
                }
                distance += value;
            },
            "down" => aim += value,
            "up" => aim -= value,
            _ => unreachable!()
        }
    }
    
    if use_aim {
        distance * depth
    } else {
        distance * aim
    }
}
