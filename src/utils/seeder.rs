use std::sync::RwLock;
use rand::{Rng,SeedableRng};
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::rngs::StdRng;

lazy_static! {
  static ref SEED: RwLock<Option<u64>> = RwLock::new(None);
}

pub fn set_seed(i: u64) {
  let mut guard = SEED.write().unwrap();
  *guard = Some(i);
}

pub fn get_rng() -> StdRng {
  let guard = SEED.read().unwrap();
  
  let rng = match *guard {
    Some(i) => {
      StdRng::seed_from_u64(i)
    },
    None => {
      StdRng::from_entropy()
    },
  };

  return rng;
}

pub fn get_value_from_range<T, R>(range: R) -> T 
where 
  T: SampleUniform,
  R: SampleRange<T>
{
  let mut r = get_rng();
  let val = r.gen_range(range);
  val
}
