  // const DIMENSION: u8 = 16385;
use rand::prelude::*;

const DIMENSION: usize = 33;

pub fn create_ds() -> [u32; DIMENSION] {
  generate_ds()
}

pub fn generate_ds() -> [u32; DIMENSION] {
  let mut rng = thread_rng();
  let mut array: [u32; DIMENSION] = [0; DIMENSION];
  array[0] = rng.gen_range(0, 1000);
  array[DIMENSION - 1] = rng.gen_range(0, 1000);

  array[17] = ((array[0] + array[DIMENSION - 1]) / 2) + 1 + rng.gen_range(0, 1000);
  array[9] = ((array[0] + array[17]) / 2) + 1 + rng.gen_range(0, 1000);

  array
}
