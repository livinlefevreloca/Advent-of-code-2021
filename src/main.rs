mod utils;

use std::{env, error::Error};
use utils::read_line_delimited;

fn main() -> Result<(), Box<dyn Error + 'static>>{
    
    let args = env::args().collect::<Vec<String>>();
    let problem: u32 = args[1].parse().map_err(|err| format!("{}: {}", err, args[1]))?;
    
    match problem {
        1 => {
            let mut increases = count_windowed_increases(1);
            println!("Found {} increases", increases);

            increases = count_windowed_increases(3);
            println!("Found {} Summed increases", increases)
        }
        _ => println!("Problem {} not implemnted yet", problem)
    };

    Ok(())
}


// Count the number of "increases" between windows of given size
//
//  Args:
//      window_size: The size of the windows to compare     
//  Returns:
//      A count for the number of increases between windows of the given size
//
//  Example:
//      with a window size of 3
// 2
// |
// 3 -- | window1, sum = 3 + 4 + 5 = 12
// |    | 
// 4 -- | -- | 
// |    |    |
// 5 -- |    |
// |         |
// 1 -- - -- |
//
fn count_windowed_increases(window_size: usize) -> u32 {
    let inputs = read_line_delimited("p1_2.txt");

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
