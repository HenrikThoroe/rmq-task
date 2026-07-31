use super::Rmq;

pub struct Store {
    n: usize,
    results: Vec<u64>,
}

impl<'a> Rmq<'a> for Store {
    fn name() -> String {
        "CachedQuery".to_string()
    }

    fn max_n() -> usize {
        10_000
    }

    fn build(data: &'a [u64]) -> Self {
        let n = data.len();
        let mut results = Vec::with_capacity(n * (n + 1) / 2);

        for l in 0..n {
            let mut min = u64::MAX;
            for &x in &data[l..] {
                min = min.min(x);
                results.push(min);
            }
        }

        Self { n, results }
    }

    fn space(&self) -> usize {
        std::mem::size_of_val(self) + self.results.len() * std::mem::size_of::<u64>()
    }

    fn query(&self, l: usize, r: usize) -> u64 {
        debug_assert!(l <= r && r < self.n);

        // The index of the result for the range [l, r] in the flattened results vector.
        // The number of elements before the index is the sum of the lengths of all
        // ranges starting at indices 0 to l-1, plus the offset (r - l) in the range starting at index l.
        let idx = l * (2 * self.n - l + 1) / 2 + (r - l);
        self.results[idx]
    }
}
