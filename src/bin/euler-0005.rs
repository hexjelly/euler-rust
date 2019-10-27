/// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
///
/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn smallest_num_divisible_by_range(min: usize, max: usize) -> u64 {
  let mut n = max;
  loop {
    if (min..=max).all(|p| n % p == 0) {
      return n as u64;
    }
    n += 1;
  }
}

fn main() {
  println!("{}", smallest_num_divisible_by_range(1, 20));
}

#[test]
fn euler_0005() {
  assert_eq!(smallest_num_divisible_by_range(1, 10), 2520);
}
