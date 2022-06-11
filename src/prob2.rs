use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct Window {
    a: i32,
    b: i32,
    c: i32
}

impl Window {
    fn sum (&self) -> i32 {
        self.a + self.b + self.c
    }
}

fn has_increased(depth1: i32, depth2: i32) -> i32 {
    if depth2 > depth1 {return 1;}
    0
}

pub fn problem2() -> io::Result<()> {
    let file = File::open("problem2.txt")?;
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
    for idx in 3..ndepths {
        let prev_window = Window{a: depths[idx-3], b: depths[idx-2], c: depths[idx-1]};
        let curr_window = Window{a: depths[idx-2], b: depths[idx-1], c: depths[idx]};
        count += has_increased(prev_window.sum(), curr_window.sum());
    }
    println!("Number of increases: {}", count);
    Ok(())
}
