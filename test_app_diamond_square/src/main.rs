extern crate diamond_square_lib;
use diamond_square_lib::diamond_square;

fn main() {
    println!("Main Test");
    let generated_data = diamond_square::create_ds();
    diamond_square::print_midpoint_displaced_array(generated_data);
}