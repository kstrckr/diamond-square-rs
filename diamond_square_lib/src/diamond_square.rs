  // const DIMENSION: u8 = 16385;
  use std::{thread, time};
  use rand::prelude::*;

const TWO: u32 = 2;

// const N: u32 = 14;
// const DIMENSION: usize = 16385;

const N: u32 = 9;
const DIMENSION: usize = 513;

pub fn create_ds() -> [u32; DIMENSION] {
  generate_ds(50)
}

pub fn generate_ds(upper_bound: u32) -> [u32; DIMENSION] {
  // randomness baseline
  let mut rng = thread_rng();
  let mut array: [u32; DIMENSION] = [0; DIMENSION];
  array[0] = rng.gen_range(0, upper_bound);
  array[DIMENSION - 1] = rng.gen_range(0, upper_bound);

  let highest_index = TWO.pow(N);

  if highest_index + 1 != DIMENSION as u32 {
    panic!("N {} and Dimension {} are incorrect", N, DIMENSION)
  }

  for i in 1..N + 1 {
    let pow = TWO.pow(i);
    for j in 1..pow {
      let index_offset = highest_index / pow;
      if j % 2 > 0 {
        let target_index = (highest_index / pow) * j;
        let lower_source = target_index - index_offset;
        let upper_source = target_index + index_offset;
        array[target_index as usize] = ((&array[lower_source as usize] + &array[upper_source as usize]) / 2) + rng.gen_range(0, (upper_bound - i) / 2);
      }
    }
  }
  array
}

pub fn print_midpoint_displaced_array(arr: [u32; DIMENSION]) {
  // println!("{:?}", arr);
  let ten_millis = time::Duration::from_millis(10);
  let max: &u32 = arr.iter().max().unwrap();
  for i in 0..DIMENSION {
    let mut printed_row: String = String::new();
    for j in 0..arr[i] {
      printed_row.push('*');
    }
    println!("{}", printed_row);
    thread::sleep(ten_millis);
  }
}
