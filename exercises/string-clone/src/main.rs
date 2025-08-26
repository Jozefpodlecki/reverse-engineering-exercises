use std::io;

use rand::{distr::Alphabetic, rng, Rng};

fn random_string(length: usize) -> String {
    (0..length)
        .map(|_| rng().sample(Alphabetic) as char)
        .collect()
}

fn main() {
    
    let mut collector = vec![];
    let times = rng().random_range(5..10);

    for _ in 0..times {
        let str = random_string(10);
        collector.push(str.clone());
        println!("{str}");
    }
}