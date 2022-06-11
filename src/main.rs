use crate::prob1::problem1;
use crate::prob2::problem2;

pub mod prob1;
pub mod prob2;

fn main() {
    println!("Problem 1");
    problem1().unwrap();
    println!("Problem 2");
    problem2().unwrap();
}
