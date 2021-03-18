extern crate diamond_square_lib;
use diamond_square_lib::diamond_square::create_ds;

fn main() {
    println!("Main Test");
    let generated_data = create_ds();
    println!("{:?}", generated_data);
}