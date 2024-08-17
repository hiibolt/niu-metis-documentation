# 2.2. Building a CUDA Project from the Ground Up
*You can find the code mentioned in this chapter [in this book's repository](https://github.com/hiibolt/niu-metis-documentation/tree/main/projects/cuda/cuda_on_metis)!*

The next part of this introductory chapter will teach you how to build, compile, and run a CUDA program from the ground up on Metis.

CUDA stands for Compute Unified Device Architecture, and it is proprietary NVIDIA-distributed software that allows developers to perform matrice-based operations at unbelievable speeds using the heavily optimized CUDA cores found only on NVIDIA GPUs.

This chapter will teach you how to run CUDA code on Metis, but it will not teach you how to write it. There are many fantastic resources on how to write it, some of which are linked below:
- [(NVIDIA) An Even Easier Introduction to CUDA](https://developer.nvidia.com/blog/even-easier-introduction-cuda/)
- [(cuda-tutorial) Introduction to CUDA](https://cuda-tutorial.readthedocs.io/en/latest/tutorials/tutorial01/)
- [(NVIDIA) CUDA Runtime API Reference](https://docs.nvidia.com/cuda/cuda-runtime-api/index.html)
- [(NVIDIA) CUDA Driver API Reference](https://docs.nvidia.com/cuda/cuda-driver-api/index.html)

## Goals
- Learn how to use the `module` commands on the login node
- Learn how to use the `qstat` command to view a running or completed job

## CUDA Boilerplate
If you did not in the previous section, start by creating a folder for our projects, then a folder for CUDA projects, and finally a folder for this project:
```bash
$ mkdir ~/projects
$ mkdir ~/projects/cuda
$ mkdir ~/projects/cuda/cuda_on_metis
$ cd ~/projects/cuda/cuda_on_metis
```

Let's start by creating a `main.cu` file with the following contents:
```c++
#include <iostream>

/// A kernel function designed to calculate the number of
///  numbers divisible by two, three, and five
///
/// # Arguments
/// * `d_number_of_divisible_by_two` - The number of numbers divisible by two
/// * `d_number_of_divisible_by_three` - The number of numbers divisible by three
/// * `d_number_of_divisible_by_five` - The number of numbers divisible by five
__global__ void calculate(
    unsigned long long int * d_number_of_divisible_by_two,
    unsigned long long int * d_number_of_divisible_by_three,
    unsigned long long int * d_number_of_divisible_by_five
) {
    int grid_x = blockIdx.x;
    int grid_y = blockIdx.y;
    int grid_z = blockIdx.z;

    int block_x = threadIdx.x;
    int block_y = threadIdx.y;
    int block_z = threadIdx.z;

    unsigned long long local_counter = 
        (grid_z * 100 * 100 * 10 * 10 * 10) + 
        (grid_y * 100 * 10 * 10) + 
        (grid_x * 10 * 10) +
        (block_z * 10 * 10) +
        (block_y * 10) +
        block_x + 1;

    unsigned long one = 1;

    if (local_counter % 2 == 0) {
        atomicAdd(d_number_of_divisible_by_two, one);
    }
    if (local_counter % 3 == 0) {
        atomicAdd(d_number_of_divisible_by_three, one);
    }
    if (local_counter % 5 == 0) {
        atomicAdd(d_number_of_divisible_by_five, one);
    }
}

int main() {
    // Say hello to the user
    std::cout << "Hello, Metis!" << std::endl;

    // Host variables
    unsigned long long int h_number_of_divisible_by_two   = 0;
    unsigned long long int h_number_of_divisible_by_three = 0;
    unsigned long long int h_number_of_divisible_by_five  = 0;

    // Device variables
    unsigned long long int * d_number_of_divisible_by_two;
    unsigned long long int * d_number_of_divisible_by_three;
    unsigned long long int * d_number_of_divisible_by_five;

    // Allocate memory on the device with the correct sizing
    cudaMalloc( &d_number_of_divisible_by_two,   sizeof(unsigned long long int) );
    cudaMalloc( &d_number_of_divisible_by_three, sizeof(unsigned long long int) );
    cudaMalloc( &d_number_of_divisible_by_five,  sizeof(unsigned long long int) );

    // Copy the memory from the host to the device
    cudaMemcpy( d_number_of_divisible_by_two,   &h_number_of_divisible_by_two,   
        sizeof(unsigned long long int), cudaMemcpyHostToDevice );
    cudaMemcpy( d_number_of_divisible_by_three, &h_number_of_divisible_by_three,
        sizeof(unsigned long long int), cudaMemcpyHostToDevice );
    cudaMemcpy( d_number_of_divisible_by_five,  &h_number_of_divisible_by_five,
        sizeof(unsigned long long int), cudaMemcpyHostToDevice );

    // Define our grid's dimensions
    dim3 gridDim(100, 100, 10);

    // Define each block's dimensions
    dim3 blockDim(10, 10, 10);

    // Run our calculation
    calculate<<<gridDim, blockDim>>>(d_number_of_divisible_by_two, d_number_of_divisible_by_three, d_number_of_divisible_by_five);
    cudaDeviceSynchronize();

    // Copy the memory back to our machine
    cudaMemcpy(&h_number_of_divisible_by_two, d_number_of_divisible_by_two, sizeof(unsigned long long int), cudaMemcpyDeviceToHost);
    cudaMemcpy(&h_number_of_divisible_by_three, d_number_of_divisible_by_three, sizeof(unsigned long long int), cudaMemcpyDeviceToHost);
    cudaMemcpy(&h_number_of_divisible_by_five, d_number_of_divisible_by_five, sizeof(unsigned long long int), cudaMemcpyDeviceToHost);

    // Provide our results to the user
    std::cout << std::endl
              << "- Numbers divisible by two: "       << h_number_of_divisible_by_two       << std::endl
              << "- Numbers divisible by three: "     << h_number_of_divisible_by_three     << std::endl
              << "- Numbers divisible by five: "      << h_number_of_divisible_by_five      << std::endl;

    // Free the memory
    cudaFree(d_number_of_divisible_by_two);
    cudaFree(d_number_of_divisible_by_three);
    cudaFree(d_number_of_divisible_by_five);

    return 0;
}
```

This program does the exact same thing as the previous section, with one key difference - it makes use of the CUDA runtime.

Instead of using indicied loops, we run our program using the compute systems of CUDA.
- Our outer loop's dimensions are replaced by the CUDA (thread) block grid, 1-3D grid containing (thread) block.
- Our inner loop's dimensions are replaced by the CUDA thread block, which are a 1-3D block containing the threads our kernel function will be executed on.

In our program, we use the maximum number of dimensions, effectively creating a 6D matrice. Because each each block is aware of its coordinates on the grid it lies on, and each thread the coordinates of the block it sits in, we can use sneaky math to calculate which number the old "counter" variable each of the *ten billion* threads translates to.

If you would like to learn more about CUDA, the resources in the introductory section of this paragraph are greatly recommended.

## Installing Modules on the Login Node
However, unlike our previous project which used `g++`, the CUDA compiler, `nvcc`, is not pre-installed. 

To install it, we will use the `module` commands mentioned briefly in the previous section.

First, let's list the modules related to `cuda` with the following command:
```bash
$ module av cuda
-------------------------------------------------- /etc/modulefiles --------------------------------------------------
cuda/cuda-7.5  cuda/cuda-8.0  cuda/cuda-11.5  cuda/cuda-11.8  cuda/cuda-11.8-rocky8  cuda/cuda-12.2
```

We see a variety of versions. For the sake of this guide, we will be using `cuda/cuda-11.8`. 

Next, let's clean up our modules, and install CUDA:
```bash
$ module purge
$ module load cuda/cuda-11.8
$ module list
Currently Loaded Modulefiles:
 1) cuda/cuda-11.8
```

Finally, we're ready to go! Let's compile and run our program:
```bash
$ nvcc -o hello_world main.cu
$ ./hello_world
Hello, Metis!

- Numbers divisible by two: 50000000
- Numbers divisible by three: 33333333
- Numbers divisible by five: 20000000
```

You will notice a nearly instantaneous completion time, versus the 20-30 seconds of the previous C version. 

Such is the power of graphical programming!

## Launching a CUDA Program with PBS
For the most part, the `run.pbs` file will look the same to the version from the previous chapter.

Create a `run.pbs` file with the following contents:
```bash
#!/bin/bash

#PBS -N hello_world_cuda
#PBS -j oe

#Note - on Metis
#              Nchunks<=32, for GPU chunks
#              Nchunks<=4096/Ncpus for CPU-only chunks
#              (run 'shownodes' command to find the number of free cpus)
#              Ncpus<=128, the total number of CPUs per node is 128
#              NPmpi<=Ncpus, the total number of CPUs allocated for MPI tasks,
#                              request NPmpi=Ncpus for non-OPENMP jobs
#              Ngpus==1,  the total number of GPUs per node is 1
#              X<=256,  28 of 32 Metis modes have 256 GB of RAM
#                       special jobs can request up to 1024 GB of RAM (4 nodes)
#
# Below, we request two chunks;
#  each chunk needs 8 CPUs, 8 MPI processes, 1 GPU card, and 16 GB RAM
#PBS -l select=1:ncpus=1:mpiprocs=1:ngpus=1:mem=2gb
#PBS -l walltime=00:15:00

# When to send a status email ("-m abe" sends e-mails at job abort, begin, and end)
#--PBS -m ae
#--#PBS -M account@niu.edu

# Navigate to our working directory
PROJECT_DIRECTORY=/home/<your_account_username>/projects/cuda/cuda_on_metis
echo "The job's working directory is $PROJECT_DIRECTORY"
cd $PROJECT_DIRECTORY

# Install GCC
echo ""
echo "Loading CUDA"
module purge; module load cuda/cuda-11.8; module load gcc/gcc-11.3.0
module list
echo "Done!"

# Compile our code
echo ""
echo "Compiling code..."
nvcc -o hello_world main.cu
echo "Done!"

# Run our binary
echo ""
echo "Executing binary..."
./hello_world
echo "Done!"

# Clean up our binary
rm ./hello_world
```

There are a few notable differences.
- Our project name is `hello_world_cuda` instead of `hello_world`.
- Our project directory is `.../hello_world_cuda` instead of `.../hello_world`.
- Instead of loading GCC, we loaded CUDA (`module load cuda/cuda-11.8`).
- Instead of compiling with G++ (`g++ -o hello_world main.cpp`), we compiled with CUDA (`nvcc -o hello_world main.cu`).

Be sure to replace any instances of `<your_account_username>` with your Metis username!

## Launching our Job with PBS
We're ready to go! All that's left is to start our job, which can be done easily with the following command:
```bash
$ qsub run.pbs
```

The output will look something like this:
```bash
18681.cm
```

This tells us the id number of our job. Wait around 30 seconds for the job to finish, and list the contents of the directory!
```bash
$ ls
hello_world_cuda.o18681 main.cu run.pbs
```

Reading the output from our job:
```
$ cat hello_world.o18681
The job's working directory is /home/<your_account_username>/projects/cuda/cuda_on_metis

Loading GCC...
Currently Loaded Modulefiles:
 1) gcc/gcc-12.3.0  
Done!

Compiling code...
Done!

Executing binary...
Hello, Metis!

- Numbers divisible by two: 5000000000
- Numbers divisible by three: 3333333333
- Numbers divisible by five: 2000000000
Done!
```

It's also worth noting that you can use the `qstat` command to view the status of a job:
```
$ qstat -x 18681
Job id            Name             User              Time Use S Queue
----------------  ---------------- ----------------  -------- - -----
18681.cm          hello_world      z1994244          00:00:02 F short 
```

The `-x` flag means you will recieve output even if the job has concluded. 

The documentation for this command, as well as `qsub`, can be found below:
- [(jlab) Documetation: `qsub`](https://www.jlab.org/hpc/PBS/qsub.html)
- [(jlab) Documetation: `qstat`](https://www.jlab.org/hpc/PBS/qstat.html)

There are also other useful commands such as `qdel` (terminates a job):
- [(jlab) Documentation: `qdel`](https://www.jlab.org/hpc/PBS/qdel.html)

## Closing Thoughts
Once again, congratulations! You have just harnessed the power of the NVIDIA hardware on Metis. 

The boilerplate from this project will be enough to get almost any CUDA project up and running. For those who recieve enough of a performance improvement to satisfy your needs, you may be able to stop here.

For tasks that require even further optimization, Metis supports [OpenMPI](https://www.open-mpi.org/), a message passing interface which allows for massively parallel computation across multiple CPUs/Metis nodes. 

Metis has modules containing GCC, CUDA, and OpenMPI for your convenience:
```bash
$ module av openmpi
-------------------- /etc/modulefiles ---------------------
openmpi/openmpi-1.8.8-gcc-11.4.0            
openmpi/openmpi-4.0.7-gcc-9.5.0-cuda-11.8   
openmpi/openmpi-4.1.1-gcc-11.3.0-cuda-11.8  
openmpi/openmpi-4.1.5-gcc-8.5.0-cuda-11.8   
openmpi/openmpi-4.1.5-gcc-11.4.0-cuda-11.8  
openmpi/openmpi-4.1.5-gcc-12.3.0-cuda-12.2  
openmpi/openmpi-4.1.6-gcc-11.4.0-cuda-11.8
```

Using a combination of both OpenMPI for coordinating large-scale tasks across many processors and CUDA for handling tasks best accelerated by GPU programming will allow you to fully harness the hardware of Metis.