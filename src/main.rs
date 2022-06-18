use crate::prob1::problem1;
use crate::prob2::problem2;
use crate::prob3::problem3;
use crate::prob4::problem4;

pub mod prob1;
pub mod prob2;
pub mod prob3;
pub mod prob3b;
pub mod prob4;

fn main() {
    println!("Problem 1");
    problem1().unwrap();
    println!("Problem 2");
    problem2().unwrap();
    println!("Problem 3");
    problem3().unwrap();
    println!("Problem 3b");
    prob3b::problem3().unwrap();
    println!("Problem 4");
    problem4().unwrap();
}
