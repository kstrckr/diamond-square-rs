  // const DIMENSION: u8 = 16385;
use rand::prelude::*;

const DIMENSION: usize = 17;
const two: u32 = 2;

pub fn create_ds() -> [u32; DIMENSION] {
  generate_ds()
}

pub fn generate_ds() -> [u32; DIMENSION] {
  let mut rng = thread_rng();
  let mut array: [u32; DIMENSION] = [0; DIMENSION];
  array[0] = rng.gen_range(0, 1000);
  array[DIMENSION - 1] = rng.gen_range(0, 1000);

  let highest_index = 16;

  for i in 1..4 + 1 {
    let pow = two.pow(i);
    println!("iteration: {} - power: {}", i, pow);
    for j in 1..pow {
      let index_offset = highest_index / pow;
      // println!("j: {} : targetIndex: {}/{}", j, j, pow);
      // println!("for J = {} : targetIndex = {}", j, (highest_index / pow) * j);
      if j % 2 > 0 {
        let target_index = (highest_index / pow) * j;
        let lower_source = target_index - index_offset;
        let upper_source = target_index + index_offset;
        // println!("for J = {} : targetIndex = {}", j, (highest_index / pow) * j);
        // println!("{} = ({}, {})", target_index, lower_source, upper_source)
        array[target_index as usize] = (&array[lower_source as usize] + &array[upper_source as usize]) / 2;
      }
    }
    println!("{:?}", array);
  }

  array
}
