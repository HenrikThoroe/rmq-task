use super::Rmq;

pub struct SparseTable {
    n: usize,
    table: Vec<u64>, // level j occupies sp[j*n .. (j+1)*n]
}

impl SparseTable {
    pub fn heap_bytes(&self) -> usize {
        self.table.len() * std::mem::size_of::<u64>()
    }
}

impl<'a> Rmq<'a> for SparseTable {
    fn name() -> String {
        "SparseTable".to_string()
    }

    fn max_n() -> usize {
        10_000_000
    }

    fn build(data: &'a [u64]) -> Self {
        let n = data.len();
        let levels = n.ilog2() as usize + 1;

        // We are allocating a little too much memory, but it does not change
        // the asymptotic space.
        let mut table = vec![0u64; levels * n];

        // 0th level: Window of size 1
        table[..n].copy_from_slice(data);

        for l in 1..levels {
            // 2^l * 1/2
            let half = 1usize << (l - 1);

            // The number of elements in the current level is n - 2^l + 1
            let len = n - (1 << l) + 1;

            // Split table so that cur starts where the next level is written
            let (prev, cur) = table.split_at_mut(l * n);

            // Only look at the previous level and discard the levels before that
            let prev = &prev[(l - 1) * n..];

            for (out, (&x, &y)) in cur[..len]
                .iter_mut()
                .zip(prev[..len].iter().zip(&prev[half..half + len]))
            {
                // out: The current element in the new level
                // x: The corresponding element in the previous level
                // y: The element in the previous level that is half a window size away
                // The current level is twice the window size of the previous level, so we take the minimum of the two halves
                // The minimum of the two elements in the previous level is the minimum of the current window
                *out = x.min(y);
            }
        }

        Self { n, table }
    }

    fn space(&self) -> usize {
        std::mem::size_of_val(self) + self.heap_bytes()
    }

    fn query(&self, l: usize, r: usize) -> u64 {
        let j = (r - l + 1).ilog2() as usize;
        let base = j * self.n;
        self.table[base + l].min(self.table[base + r + 1 - (1 << j)])
    }
}
