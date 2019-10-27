use euler::primes::inc_sieve;

/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
///
/// What is the 10 001st prime number?
fn find_nth_prime(n: usize) -> u64 {
  inc_sieve().take(n).last().unwrap()
}

fn main() {
  println!("{}", find_nth_prime(10001));
}

#[test]
fn euler_0007() {
  assert_eq!(find_nth_prime(6), 13);
}
