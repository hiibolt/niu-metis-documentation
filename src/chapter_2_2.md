# 2.2. Rust on Metis
Rust is a highly unique language that has C-level performance without C-esque memory issues.

No segfaults, no race conditions, no risk of another Therac-25.

Rust has multiple features positioning it as an ideal language for HPC:
- **Rich ecosystem**: Libaries for CUDA, parallelism, and distributed computing
- **Memory safety**: No leaking memory, no unexpected crashes at the end of your workload
- **Blistering performance**: With compiler optimizations, Rust can outperform C
- **Revealing type system**: Understand your data and its output before and after it's processed

## Chapter 2.2.1: A Parallel Rust Project, from the Ground Up
This chapter will teach you:
* The basics of Rust, tailored for HPC
* How to compile and execute Rust code with Cargo
* How to launch a Rust program with PBS
* Trivially writing parallel code with the `rayon` crate
## Chapter 2.2.2: Cross-Node Computation with MPI and Rust
This chapter will teach you:
* How to prepare your environment for MPI development
* The basics of MPI with openMPI as a backend
* How to use the `mpi` crate in Rust for cross-node computation and communication