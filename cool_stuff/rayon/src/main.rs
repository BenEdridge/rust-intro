use rayon::prelude::*;
use std::time::Instant;
use rand::{distributions::Uniform, Rng};

fn main() {

    // Used for our random large vector below
    let mut rng = rand::thread_rng();
    let range = Uniform::new(1, 10);

    let large_vector: Vec<u64> = (0..10_000_000)
        .map(|_| rng.sample(&range))
        .collect();

    let start = Instant::now();
    let sum1 = sum_of_squares(&large_vector);
    let duration1 = start.elapsed();
    
    let start = Instant::now();
    let sum2 = sum_of_squares_parallel(&large_vector); // SPEED!
    let duration2 = start.elapsed();
    
    println!("Sequential Sum of Squares is: {}, {}ms", sum1, duration1.as_millis());
    println!("Parallel   Sum of Squares is: {}, {}ms", sum2, duration2.as_millis());

    println!("Speedup of: {:.2}x", duration1.as_millis() as f64/ duration2.as_millis() as f64);
}

fn sum_of_squares_parallel(input: &Vec<u64>) -> u64 {
    // -> `par_iter` is a paraller iterator provided by `rayon`
    input.par_iter().map(|&i| i * i).sum()
}

fn sum_of_squares(input: &Vec<u64>) -> u64 {
    input.iter().map(|&i| i * i).sum()
}
