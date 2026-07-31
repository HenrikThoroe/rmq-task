/// Vectorizes more reliably than `Iterator::min`, which folds through `Ord::cmp`.
#[inline]
pub fn min_of(xs: &[u64]) -> u64 {
    xs.iter().copied().fold(u64::MAX, u64::min)
}
