use std::{io::Read, path::Path};

/// The input data.
pub struct Input {
    pub data: Vec<u64>,
    pub queries: Vec<(usize, usize)>,
}

/// Read the given input file.
pub fn read_input(file: &Path) -> Input {
    let mut input = String::new();
    std::fs::File::open(file)
        .expect("Open input file")
        .read_to_string(&mut input)
        .expect("Read input file");
    let mut vals = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap());
    // First line has "{n} {q}"
    let n: usize = vals.next().unwrap() as usize;
    let q: usize = vals.next().unwrap() as usize;
    // Then n lines "{ai}"
    let data = vals.by_ref().take(n).collect();
    // Then q lines "{l} {r}"
    let queries = (0..q)
        .map(|_| (vals.next().unwrap() as usize, vals.next().unwrap() as usize))
        .collect();
    Input { data, queries }
}
