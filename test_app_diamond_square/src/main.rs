extern crate diamond_square_lib;
use diamond_square_lib::diamond_square;

use std::time::{SystemTime};

fn main() {
    println!("Main Test");
    let start = SystemTime::now();
    let mut all_data: Vec<Vec<u32>> = Vec::new();
    for i in 0..10000 {
        all_data.push(diamond_square::create_ds());
    }
    // let generated_data = diamond_square::create_ds();
    let finish = SystemTime::now();
    let difference = finish.duration_since(start)
        .expect("System clock may have self-updated during run resulting in negative duration");
    println!("{:?}", difference);
}