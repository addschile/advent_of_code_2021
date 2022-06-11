use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn has_increased(depth1: i32, depth2: i32) -> i32 {
    if depth2 > depth1 {return 1;}
    0
}

pub fn problem1() -> io::Result<()> {
    let file = File::open("problem1.txt")?;
    let reader = BufReader::new(file);
    let depths: Vec<i32> = reader.lines().
        filter(|item| item.is_ok()).
        map(|item| item.unwrap()).
        map(|item| item.parse::<i32>()).
        filter(|item| item.is_ok()).
        map(|item| item.unwrap()).
        collect();
    let ndepths = depths.len();
    let mut count = 0;
    for idx in 1..ndepths {
        let prev_depth = depths[idx-1];
        let curr_depth = depths[idx];
        count += has_increased(prev_depth, curr_depth);
    }
    println!("Number of increases: {}", count);
    Ok(())
}
