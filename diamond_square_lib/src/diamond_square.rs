  // const DIMENSION: u8 = 16385;
use rand::prelude::*;

const TWO: u32 = 2;

const N: u32 = 10;
const DIMENSION: usize = TWO.pow(N) as usize + 1;

pub fn create_ds() -> [u32; DIMENSION] {
  generate_ds()
}

pub fn generate_ds() -> [u32; DIMENSION] {
  let mut rng = thread_rng();
  let mut array: [u32; DIMENSION] = [0; DIMENSION];
  array[0] = rng.gen_range(0, 1000);
  array[DIMENSION - 1] = rng.gen_range(0, 1000);

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
        array[target_index as usize] = (&array[lower_source as usize] + &array[upper_source as usize]) / 2;
      }
    }
    println!("{:?}", array);
  }

  array
}
