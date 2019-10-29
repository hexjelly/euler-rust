use euler::primes::inc_sieve;
/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
///
/// Find the sum of all the primes below two million.
fn sum_of_primes_below(n: u64) -> u64 {
  inc_sieve().take_while(|p| *p < n).sum()
}

fn main() {
  println!("{}", sum_of_primes_below(2_000_000));
}

#[test]
fn euler_0010() {
  assert_eq!(sum_of_primes_below(10), 17);
}
