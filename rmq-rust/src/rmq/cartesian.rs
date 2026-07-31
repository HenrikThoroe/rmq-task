use super::Rmq;
use super::sparse_table::SparseTable;
use std::collections::HashMap;

pub struct CartesianTree<'a, const S: usize> {
    data: &'a [u64],

    /// Maps every block to the ID of the shape of it's cartesian tree
    block_table: Vec<u32>,

    /// Flattened table of all possible local [i, j]
    /// range queries for each unique block shape.
    /// S must be less than or equal to 32 to keep the block
    /// local index inside a single byte.
    tables: Vec<u8>,

    /// Sparse table over per block minimum values.
    sparse_table: SparseTable,
}

impl<'a, const S: usize> CartesianTree<'a, S> {
    /// Number of entries per shape table <-> number of possible local [i, j] range queries
    const STRIDE: usize = S * (S + 1) / 2;

    /// Flatened index of the [i, j] query in a block.
    #[inline]
    fn index(i: usize, j: usize) -> usize {
        debug_assert!(i <= j && j < S);
        j * (j + 1) / 2 + i
    }

    /// Global index of the minimum element inside a block.
    /// Performs a local query on block b for the range [i, j]
    /// and returns the global index of the minimum element in the
    /// global data array.
    #[inline]
    fn argmin_in_block(&self, b: usize, i: usize, j: usize) -> usize {
        let id = self.block_table[b] as usize;
        b * S + self.tables[id * Self::STRIDE + Self::index(i, j)] as usize
    }
}

impl<'a, const S: usize> Rmq<'a> for CartesianTree<'a, S> {
    fn name() -> String {
        format!("CartesianTree<{S}>")
    }

    fn max_n() -> usize {
        10_000_000
    }

    fn build(data: &'a [u64]) -> Self {
        assert!(
            (1..=32).contains(&S),
            "S <= 32 keeps the 2S-bit signature inside a u64 and positions inside a u8"
        );

        let n = data.len();
        let nb = n.div_ceil(S);

        let mut shape_ids: HashMap<u64, u32> = HashMap::new();
        let mut tables: Vec<u8> = Vec::new();
        let mut block_table = Vec::with_capacity(nb);
        let mut minima = Vec::with_capacity(nb);
        let mut stack: Vec<u64> = Vec::with_capacity(S);

        for block in data.chunks(S) {
            // Build the signature of the cartesian tree for the block.
            // The signature is a bitstring where each push contributes a 1 bit and each pop contributes a 0 bit.
            // The signature is stored in a u64, so the block size must be at most 32 to fit in 2S bits.
            stack.clear();
            let mut sig = 0u64;
            let mut bits = 0u32;
            for &x in block {
                while stack.last().is_some_and(|&t| t > x) {
                    stack.pop();
                    bits += 1;
                }
                stack.push(x);
                sig |= 1u64 << bits;
                bits += 1;
            }

            let id = *shape_ids.entry(sig).or_insert_with(|| {
                let id = (tables.len() / Self::STRIDE) as u32;
                let base = tables.len();
                tables.resize(base + Self::STRIDE, 0);

                for i in 0..block.len() {
                    let mut best = i;
                    for j in i..block.len() {
                        if block[j] < block[best] {
                            best = j;
                        }
                        tables[base + Self::index(i, j)] = best as u8;
                    }
                }
                id
            });
            block_table.push(id);

            // Use block minimum for the sparse table.
            let pos = tables[id as usize * Self::STRIDE + Self::index(0, block.len() - 1)];
            minima.push(block[pos as usize]);
        }

        Self {
            data,
            block_table,
            tables,
            sparse_table: SparseTable::build(&minima),
        }
    }

    fn space(&self) -> usize {
        std::mem::size_of_val(self)
            // + self.data.len() * std::mem::size_of::<u64>()
            + self.block_table.len() * std::mem::size_of::<u32>()
            + self.tables.len() * std::mem::size_of::<u8>()
            + self.sparse_table.heap_bytes()
    }

    fn query(&self, l: usize, r: usize) -> u64 {
        let (bl, br) = (l / S, r / S);

        // Inside single block, perform a local query on the block's table.
        if bl == br {
            return self.data[self.argmin_in_block(bl, l - bl * S, r - bl * S)];
        }

        // Perform local queries in the tail and head of the query.
        let left = self.data[self.argmin_in_block(bl, l - bl * S, S - 1)];
        let right = self.data[self.argmin_in_block(br, 0, r - br * S)];
        let mut best = left.min(right);

        // Perform a sparse table lookup up on the fully enclosed blocks
        if bl + 1 < br {
            best = best.min(self.sparse_table.query(bl + 1, br - 1));
        }

        best
    }
}
