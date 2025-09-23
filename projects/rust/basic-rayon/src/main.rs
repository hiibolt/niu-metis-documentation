use rand::prelude::*;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    // Create a vector of n random integers
    let mut nums: Vec<f64> = vec!(0.0; 100_000_000);
    rand::rng().fill(&mut nums[..]);

    // Test the parallel average
    let start = Instant::now();
    let avg = parallel_average(&nums);
    let duration = start.elapsed();
    println!("Average: {} in {:?}", avg, duration);

    // Test the sequential average
    let start = Instant::now();
    let avg = sequential_average(&nums);
    let duration = start.elapsed();
    println!("Average: {} in {:?}", avg, duration);
}

fn sequential_average ( nums: &[f64] ) -> f64 {
    nums.into_iter().sum::<f64>() / nums.len() as f64
}
fn parallel_average ( nums: &[f64] ) -> f64 {
    let sum: f64 = nums
        .par_chunks(nums.len() / 100)
        .map(|ind| {
            let mut local_sum: f64 = 0.0;
            for &num in ind {
                local_sum += num;
            }
            local_sum
        })
        .sum();

    sum / (nums.len() as f64)
}