use super::Rmq;
use super::sparse_table::SparseTable;
use super::util::min_of;

pub struct BlockRmq<'a, const S: usize> {
    data: &'a [u64],
    table: SparseTable,
}

impl<'a, const S: usize> Rmq<'a> for BlockRmq<'a, S> {
    fn name() -> String {
        format!("Block<{}>", S)
    }

    fn max_n() -> usize {
        10_000_000
    }

    fn build(data: &'a [u64]) -> Self {
        let minima: Vec<u64> = data.chunks(S).map(min_of).collect();

        Self {
            data,
            table: SparseTable::build(&minima),
        }
    }

    fn space(&self) -> usize {
        std::mem::size_of_val(self) + self.table.heap_bytes()
    }

    fn query(&self, l: usize, r: usize) -> u64 {
        let lb = (l + S - 1) / S; // first fully covered block
        let rb = (r + 1) / S; // one past last fully covered block

        if lb >= rb {
            return min_of(&self.data[l..=r]);
        }

        let mut min_val = self.table.query(lb, rb - 1);
        if l < lb * S {
            min_val = min_val.min(min_of(&self.data[l..lb * S]));
        }
        if rb * S <= r {
            min_val = min_val.min(min_of(&self.data[rb * S..=r]));
        }
        min_val
    }
}

pub struct PrecomputeBlockRmq<'a, const S: usize> {
    data: &'a [u64],
    table: SparseTable,
    suff: Vec<u64>,
    pref: Vec<u64>,
}

impl<'a, const S: usize> Rmq<'a> for PrecomputeBlockRmq<'a, S> {
    fn name() -> String {
        format!("PrecomputeBlock<{}>", S)
    }

    fn max_n() -> usize {
        10_000_000
    }

    fn build(data: &'a [u64]) -> Self {
        let n = data.len();
        let mut pref = vec![0u64; n];
        let mut suff = vec![0u64; n];

        for ((block, p), s) in data
            .chunks(S)
            .zip(pref.chunks_mut(S))
            .zip(suff.chunks_mut(S))
        {
            let mut m = u64::MAX;
            for (i, &x) in block.iter().enumerate() {
                m = m.min(x);
                p[i] = m;
            }
            let mut m = u64::MAX;
            for (i, &x) in block.iter().enumerate().rev() {
                m = m.min(x);
                s[i] = m;
            }
        }

        // The block minimum is the last prefix minimum of that block.
        let minima: Vec<u64> = pref.chunks(S).map(|c| c[c.len() - 1]).collect();

        Self {
            data,
            table: SparseTable::build(&minima),
            pref,
            suff,
        }
    }

    fn space(&self) -> usize {
        std::mem::size_of_val(self)
            + self.pref.len() * std::mem::size_of::<u64>()
            + self.suff.len() * std::mem::size_of::<u64>()
            + self.table.heap_bytes()
    }

    fn query(&self, l: usize, r: usize) -> u64 {
        let lb = (l + S - 1) / S; // first fully covered block
        let rb = (r + 1) / S; // one past last fully covered block

        if lb >= rb {
            return min_of(&self.data[l..=r]);
        }

        let min_val = self.table.query(lb, rb - 1);
        let m = self.suff[l].min(self.pref[r]);

        min_val.min(m)
    }
}
