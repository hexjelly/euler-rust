use euler::primes::inc_sieve;

/// The prime factors of 13195 are 5, 7, 13 and 29.
///
/// What is the largest prime factor of the number 600851475143 ?
fn largest_prime_factor(n: u64) -> u64 {
  if n <= 1 {
    return n;
  }
  let mut remain = n;
  let mut primes = inc_sieve();
  let mut factors = vec![];

  while remain > 1 {
    let next = primes.next().unwrap();
    if remain % next == 0 {
      factors.push(next);
      remain /= next;
    }
  }
  *factors.iter().max().unwrap()
}

fn main() {
  println!("{}", largest_prime_factor(600_851_475_143))
}

#[test]
fn euler_0003() {
  assert_eq!(largest_prime_factor(13195), 29);
}
