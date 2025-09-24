# 2.2.2. Cross-Node Computation with MPI and Rust
*You can find the code mentioned in this chapter [in this book's repository](https://github.com/hiibolt/niu-metis-documentation/tree/main/projects/rust/basic-mpi)!*

## What is OpenMPI, and How Does it Differ from OpenMP?
I'll start by loosely contrasting the functionality of the two:
- OpenMP is for **shared memory** across multiple **processors/threads**
- OpenMPI is for **distributed memory** across **multiple systems**

OpenMP is something that's semi-comparable to our previous approach to multithreading. It's a lot beefier, but the author of this documentation generally discourages the use of it - recommending to instead use either language parallelism features (for simplicity) or OpenMPI (for drastically improved performance).

## How OpenMPI Works
MPI is a protocol for a (M)essage-(P)assing (I)nterface.

There's four concepts core to MPI:
* **Universe** - The collection of all nodes. On Metis, this is abstracted away by PBS Professional, so don't think too hard about it.
* **World** - The collection of all MPI processes and their intercommunication layers. You can think of this as the "meeting room", where each process has a headset and microphone to talk to oneanother.
* **Size** - The number of MPI processes in the **World**.
* **Rank** - The index representing *this* MPI process in the **World**.

When you launch an MPI program, all programs with the same binary and memory. In fact, if you didn't use the rank indicator anywhere in the program, they'd be identical in most cases, simply duplicating output `size` times.

Here's an example in Rust that does just that:
```rust
use mpi::traits::*;
use anyhow::{Context, Result};

fn main ( ) -> Result<()> {
    let universe = mpi::initialize()
        .context("Failed to initialize MPI.")?;
    let world = universe.world();
    let size: i32 = world.size();

    println!("Size: {size}");

    Ok(())
}
```

To get this to run, you'll need to do the following:
```bash
$ mkdir -p ~/projects/rust/basic-mpi
$ cd ~/projects/rust/basic-mpi
$ cargo init .
$ cargo add anyhow
$ cargo add mpi
$ module purge
$ module load openmpi/openmpi-5.0.7-gcc-14.2.0-cuda-12.8
$ cargo run
```

For an MPI run with size 3, you'd get:
```
Size: 3
Size: 3
Size: 3
```

Neat, but why run the same thing in multiple places?

### Rank-Based Inter-process Logic and Communication
Things start to get interesting quick when you consider the `rank`:
```rust
use mpi::traits::*;
use anyhow::{Context, Result};

fn main ( ) -> Result<()> {
    let universe = mpi::initialize()
        .context("Failed to initialize MPI.")?;
    let world = universe.world();
    let size: i32 = world.size();
    let rank: i32 = world.rank();

    println!("Size: {size} - Rank: {rank}");

    Ok(())
}
```

Running this, we get a different result:
```
Size: 3 - Rank: 0
Size: 3 - Rank: 1
Size: 3 - Rank: 2
```

If you've been paying close attention, it probably just clicked why this is big - you can diverge in logic based on this rank.

Here's a full example for this logic at work:
```rust
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
```

Lots of things just got added, so let's break it down.
* Main can fail. That's why it returns a `Result<()>`, and also why the `universe` has a `.context(...)?` snippet - that wraps the error with additional information and early returns if it indeed contains an error.
* Each and every node calculates the average of `(NUM_ELEMENT / size)` elements, and then sends their result to the node with rank 0.
* The node with rank 0, and *only* that node, receives the results from each node (including itself) and prints the result.
* Since our `main` function returns a `Result<()>`, we need to finish the function by returning the `Result::Ok<()>` variant, which can be shortened to `Ok<()>` since we declared we'd be returning an `Result` in the function definition

### Building and Executing a Rust MPI binary
Now, the above steps still work - but what's the point of running this on one process?

Let's get started with a basic 2-process PBS batchfile:
```bash
#!/bin/bash
#PBS -N basic-mpi
#PBS -l select=2:ncpus=1:mpiprocs=1
#PBS -l walltime=00:10:00
#PBS -j oe
#PBS -o basic-mpi.out

# Change to the directory from which the job was submitted
cd $PBS_O_WORKDIR

# Load MPI module (adjust to your cluster's modules)
echo ""
echo "[ Loading Modules ]"
module purge
module load openmpi/openmpi-5.0.7-gcc-14.2.0-cuda-12.8

# Path to your compiled Rust binary
echo ""
echo "[ Building Program ]"
cargo build --release
BIN=./target/release/basic-mpi

# Run with 2 processes (1 per node)
echo ""
echo "[ Starting Program ]"
mpirun -np 2 -hostfile $PBS_NODEFILE $BIN
```

Let's note two things. Firstly, we aren't using `cargo run` here, we're actually building a binary with the release profile. This is important because not only is a binary required by `mpirun`, but a binary built this way is significantly better optimized.

Secondly, we must coordinate that we're asking for 2 MPI processes in **two** places: 
- The PBS directive (`select=2`)
- `mpirun` with (`-np 2`)


### Final Results
Testing this, we get a successful output:
```
$ qsub run.pbs
69937.cm
$ cat basic-mpi.out

[ Loading Modules ]
Loading openmpi/openmpi-4.1.8-gcc-11.4.0-cuda-11.8
  Loading requirement: gcc/gcc-11.4.0 cuda/cuda-11.8

[ Building Program ]
   Compiling basic-mpi v0.1.0 (/nfs/ihfs/home_metis/z1994244/projects/rust/basic-mpi)
    Finished `release` profile [optimized] target(s) in 0.42s

[ Starting Program ]
Global average of 0.4995294844859745 over 1000000 elements
Computed on 2 MPI processes
```

With this, we've successfully run a multi-node Rust program with distributed memory!

This approach is hyper-modern - you're getting the low-level performance of C and OpenMPI with the safety and opinionated predictability of Rust. 

Very interesting stuff - Rust uniquely positions itself as a potential competitor in the HPC space with the aforementioned benefits.