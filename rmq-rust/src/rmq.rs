pub mod block;
pub mod cartesian;
pub mod naive;
pub mod segment_tree;
pub mod sparse_table;
pub mod store;
mod util;

pub trait Rmq<'a> {
    fn name() -> String;
    /// To save time, only run benchmarks up to this n.
    fn max_n() -> usize {
        usize::MAX
    }
    fn build(data: &'a [u64]) -> Self;
    /// Space usage in bytes.
    fn space(&self) -> usize;
    fn query(&self, l: usize, r: usize) -> u64;
}
