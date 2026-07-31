use crate::rmq::util::min_of;

use super::Rmq;

pub struct Naive<'a> {
    data: &'a [u64],
}

impl<'a> Rmq<'a> for Naive<'a> {
    fn name() -> String {
        "Naive".to_string()
    }

    fn max_n() -> usize {
        10_000
    }

    fn build(data: &'a [u64]) -> Self {
        Self { data }
    }

    fn space(&self) -> usize {
        std::mem::size_of_val(self)
    }

    fn query(&self, l: usize, r: usize) -> u64 {
        min_of(&self.data[l..=r])
    }
}
