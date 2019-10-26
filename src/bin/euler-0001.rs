/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
///
/// Find the sum of all the multiples of 3 or 5 below 1000.
fn multiple_of_any(min: u64, max: u64, multiple_predicates: &[u64]) -> u64 {
  (min..max)
    .filter(|n| multiple_predicates.iter().any(|p| n % p == 0))
    .sum()
}

fn main() {
  println!("{}", multiple_of_any(0, 1000, &[3, 5]))
}

#[test]
fn euler_0001() {
  assert_eq!(multiple_of_any(0, 1000, &[3, 5]), 233_168);
}
