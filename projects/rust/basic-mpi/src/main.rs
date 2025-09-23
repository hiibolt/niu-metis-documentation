use mpi::traits::*;
use rand::prelude::*;
use anyhow::{Context, Result};

const NUM_ELEMENTS: usize = 1_000_000;

fn calculate_random_avg ( n: usize ) -> f64 {
    let mut arr = vec!(0.0; n);
    rand::rng().fill(&mut arr[..]);

    arr.into_iter().sum::<f64>() / (n as f64)
}
fn main ( ) -> Result<()> {
    let universe = mpi::initialize()
        .context("Failed to initialize MPI.")?;
    let world = universe.world();
    let size: i32 = world.size();
    let rank: i32 = world.rank();

    let local_elements  = NUM_ELEMENTS / (size as usize)
        + (rank == 0)
            .then_some(NUM_ELEMENTS % (size as usize))
            .unwrap_or(0);
    let local_avg = calculate_random_avg(local_elements);
    world.process_at_rank(0).send(&local_avg);

    if rank == 0 { 
        let global_avg = (0..size)
            .fold(0f64, |acc, _| {
                acc + world.any_process().receive::<f64>().0
            }) / (size as f64);

        println!("Global average of {global_avg} over {NUM_ELEMENTS} elements");
        println!("Computed on {size} MPI processes");
    }

    Ok(())
}
