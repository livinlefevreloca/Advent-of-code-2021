mod utils;

use std::{env, error::Error};
use utils::read_line_delimited;

fn main() -> Result<(), Box<dyn Error + 'static>>{
    
    let args = env::args().collect::<Vec<String>>();
    let problem: u32 = args[1].parse().map_err(|err| format!("{}: {}", err, args[1]))?;
    
    match problem {
        1 => {
            let mut increases = count_increases();
            println!("Found {} increases", increases);

            increases = count_sum_increases();
            println!("Found {} Summed increases", increases)
        }
        _ => println!("Problem {} not implemnted yet", problem)
    };

    Ok(())
}

fn count_increases() -> u32 {
    let inputs = read_line_delimited("p1.txt");

    inputs
        .into_iter()
        .map(|val| val.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .windows(2)
        .map(| window | window[1] > window[0])
        .fold(0, |acc, truth| {
            match truth {
                true => acc + 1,
                _ => acc
            }
        })
            
} 

fn count_sum_increases() -> u32 {
    let inputs = read_line_delimited("p1_2.txt");

    inputs
        .into_iter()
        .map(|val| val.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .windows(3)
        .collect::<Vec<&[u32]>>()
        .windows(2)
        .map(| window | window.to_vec())
        .collect::<Vec<Vec<&[u32]>>>()
        .into_iter()
        .map(|window| {
            window[0]
                .into_iter()
                .fold(0, |acc, num| acc + num) <
            window[1]
                .into_iter()
                .fold(0, |acc, num| acc + num)
        })
        .fold(0, |acc, val| {
            match val {
                true => acc + 1,
                _ => acc
            }
        })
        
}
