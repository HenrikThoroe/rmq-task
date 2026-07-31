mod input;
mod rmq;

use input::{Input, read_input};
use rmq::{
    Rmq, block::BlockRmq, block::PrecomputeBlockRmq, cartesian::CartesianTree, naive::Naive,
    segment_tree::SegmentTree, sparse_table::SparseTable, store::Store,
};
use std::path::PathBuf;

/// Bench the given RMQ implementation on the given input, and print the results in CSV format.
fn bench<'a, RMQ: Rmq<'a>>(input: &'a Input) {
    eprint!("{:>10}\t{:>30}\t", input.data.len(), RMQ::name());
    if input.data.len() > RMQ::max_n() {
        eprintln!("skipped");
        return;
    }

    let rmq = RMQ::build(&input.data);
    eprint!("{:>10}\t", rmq.space());
    let start = std::time::Instant::now();
    let mut sum = 0;
    for &(l, r) in &input.queries {
        sum += rmq.query(l, r);
    }
    let elapsed = start.elapsed().as_nanos() as f64 / input.queries.len() as f64;
    println!(
        "{},{},\"{}\",{},{},{}",
        input.data.len(),
        input.queries.len(),
        RMQ::name(),
        rmq.space(),
        sum,
        elapsed
    );
    eprintln!("{:>3}\t{:>8.2}ns/q", sum % 1000, elapsed);
}

fn main() {
    println!("n,q,name,space,sum,time");

    let file_or_dir = PathBuf::from(std::env::args().nth(1).expect("Usage: bench <input_dir>"));

    eprintln!("Reading input from \"{}\" ..", file_or_dir.display());
    let mut inputs = vec![];
    if file_or_dir.is_file() {
        inputs.push(read_input(&file_or_dir));
    } else {
        for entry in file_or_dir.read_dir().expect("Read input directory") {
            if let Ok(file) = entry {
                if Some("in") == file.path().extension().and_then(|s| s.to_str()) {
                    let input = read_input(&file.path());
                    inputs.push(input);
                }
            }
        }
        inputs.sort_by_key(|input| input.data.len());
    }
    for input in inputs {
        bench::<Naive>(&input);
        bench::<Store>(&input);
        bench::<SparseTable>(&input);
        bench::<SegmentTree>(&input);
        bench::<BlockRmq<4>>(&input);
        bench::<BlockRmq<64>>(&input);
        bench::<BlockRmq<512>>(&input);
        bench::<PrecomputeBlockRmq<4>>(&input);
        bench::<PrecomputeBlockRmq<64>>(&input);
        bench::<PrecomputeBlockRmq<512>>(&input);
        bench::<CartesianTree<6>>(&input);
        bench::<CartesianTree<8>>(&input);
        bench::<CartesianTree<16>>(&input);
        bench::<CartesianTree<32>>(&input);
    }
}
