use super::Rmq;

pub struct SegmentTree {
    n: usize,
    tree: Vec<u64>,
}

impl<'a> Rmq<'a> for SegmentTree {
    fn name() -> String {
        "SegmentTree".to_string()
    }

    fn max_n() -> usize {
        10_000_000
    }

    fn build(data: &'a [u64]) -> Self {
        let n = data.len();
        let mut tree = vec![0u64; 2 * n];
        tree[n..].copy_from_slice(data);

        // The end index of the next looked at segment
        let mut hi = n;
        while hi > 1 {
            // The start index of the current segment
            let lo = (hi + 1) / 2;

            // Tail contains the child nodes of the current level, head[lo..hi] contains the current level
            let (head, tail) = tree.split_at_mut(hi);
            for (dst, pair) in head[lo..hi]
                .iter_mut()
                .zip(tail[2 * lo - hi..].chunks_exact(2))
            {
                // Split tail into pairs of two and take the minimum of each pair to fill the current level
                *dst = pair[0].min(pair[1]);
            }
            hi = lo;
        }

        Self { n, tree }
    }

    fn space(&self) -> usize {
        std::mem::size_of_val(self) + self.tree.len() * std::mem::size_of::<u64>()
    }

    fn query(&self, l: usize, r: usize) -> u64 {
        let mut res = u64::MAX;

        // Offset l and r to start inside the leaves of the tree
        let mut l = l + self.n;
        let mut r = r + self.n + 1;

        while l < r {
            let a = self.tree[l];
            let b = self.tree[r - 1];

            // The masks are 0 if the last bit of l/r are set (i.e. l/r are odd).
            // Otherwise all bits are set in the mask.
            let l_mask = ((l & 1) as u64).wrapping_sub(1);
            let r_mask = ((r & 1) as u64).wrapping_sub(1);

            // If l is odd, we take the value at l and move to the next node.
            // If r is odd, we take the value at r-1 and move to the previous node.
            res = res.min(a | l_mask);
            res = res.min(b | r_mask);

            // Move up the tree by dividing l and r by 2.
            // We add 1 to l before dividing to ensure that we do not move to the left
            // parent (outside the range) if l is odd.
            l = (l + 1) >> 1;
            r >>= 1;
        }
        res
    }
}
