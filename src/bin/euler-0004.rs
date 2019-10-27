use std::collections::BinaryHeap;

/// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.

fn largest_palindrome(digits: usize) -> Option<u64> {
  let max: u32 = "9".repeat(digits).parse().unwrap();
  let min: u32 = ("1".to_owned() + &"0".repeat(digits - 1)).parse().unwrap();
  let mut palindromes = BinaryHeap::new();

  for n in (min..=max).rev() {
    for m in (min..=max).rev() {
      let p = (n * m).to_string();
      if p == p.chars().rev().collect::<String>() {
        palindromes.push(p.parse::<u64>().unwrap());
      }
    }
  }
  palindromes.peek().cloned()
}

fn main() {
  println!("{}", largest_palindrome(3).unwrap());
}

#[test]
fn euler_0004() {
  assert_eq!(largest_palindrome(2), Some(9009));
}
