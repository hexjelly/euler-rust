/// The sum of the squares of the first ten natural numbers is,
///
/// 12 + 22 + ... + 102 = 385
/// The square of the sum of the first ten natural numbers is,
///
/// (1 + 2 + ... + 10)2 = 552 = 3025
/// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
///
/// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn sum_square_difference(min: u64, max: u64) -> u64 {
  let sum_of_squares: u64 = (min..=max).map(|n| n.pow(2)).sum();
  let square_of_sum: u64 = (min..=max).sum::<u64>().pow(2);
  (square_of_sum - sum_of_squares)
}

fn main() {
  println!("{}", sum_square_difference(1, 100));
}

#[test]
fn euler_0006() {
  assert_eq!(sum_square_difference(1, 10), 2640);
}
