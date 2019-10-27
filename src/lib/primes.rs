use std::collections::BTreeMap;

struct IncSieve {
  sieve: BTreeMap<u64, u64>,
  last: Option<u64>,
}

impl Iterator for IncSieve {
  type Item = u64;

  fn next(&mut self) -> Option<u64> {
    let mut candidate = self.last.map_or(2, |n| n + 1);
    loop {
      // not prime, get smallest factor, add new multple
      if let Some(smallest_factor) = self.sieve.get(&candidate).cloned() {
        self.sieve.remove(&candidate);

        let mut next_multiple = candidate + smallest_factor;
        while self.sieve.contains_key(&next_multiple) {
          next_multiple += smallest_factor;
        }
        self.sieve.insert(next_multiple, smallest_factor);
      } else {
        // found prime, add its square to sieve
        self.sieve.insert(candidate * candidate, candidate);
        self.last = Some(candidate);
        return Some(candidate);
      }
      candidate += 1;
    }
  }
}

/// Incremental sieve prime generator
pub fn inc_sieve() -> impl Iterator<Item = u64> {
  IncSieve {
    sieve: BTreeMap::new(),
    last: None,
  }
}

#[cfg(test)]
mod test_super {
  use super::*;

  #[test]
  fn inc_sieve_works() {
    let prime_nums: Vec<_> = inc_sieve().take(5).collect();
    assert_eq!(prime_nums, &[2, 3, 5, 7, 11])
  }
}
