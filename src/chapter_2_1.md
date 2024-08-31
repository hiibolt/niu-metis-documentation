# 2.1. Building a C++ Project from the Ground Up
*You can find the code mentioned in this chapter [in this book's repository](https://github.com/hiibolt/niu-metis-documentation/tree/main/projects/cpp/cpp_on_metis)!*

This introductory project will teach you the absolute minimal nessecary information to create a basic C++ project on the Metis supercomputer.

Before we tackle more robust and complex technologies such as CUDA or OpenMPI, our goal is to familiarize ourselves with Metis before abstracting and building upon our understanding. 

We'll instead opt to start with the most basic of programs - "Hello, World" (with, of course, a computationally intensive task) - to get started!

## Goals
* Get a feel for the `module` commands
* Get a feel for the [PBS Professional](https://altair.com/pbs-professional) job submission system
* Understand the layout of a `.pbs` job script file
* Get a feel for the `qsub` command

## C++ Boilerplate
First, let's start by creating a folder for our projects, then a folder for C++, and finally a folder for this project:
```bash
$ mkdir /lstr/sahara/<your_project>/<you>
$ mkdir /lstr/sahara/<your_project>/<you>/cpp
$ mkdir /lstr/sahara/<your_project>/<you>/cpp/cpp_on_metis
$ cd /lstr/sahara/<your_project>/<you>/cpp/cpp_on_metis
```
Let's start by creating a `main.cpp` file with the following contents:
```c++
#include <iostream>

int main () {
    // Say hello to the user
    std::cout << "Hello, Metis!" << std::endl;

    // Initialize our counter variables
    unsigned long long int counter = 0;
    unsigned long long int number_of_divisible_by_two = 0;
    unsigned long long int number_of_divisible_by_three = 0;
    unsigned long long int number_of_divisible_by_five = 0;

    // First, iterate through a 3D grid to get to our block
    for ( int grid_z = 0; grid_z < 1000; grid_z++ ) {
        for ( int grid_y = 0; grid_y < 100; grid_y++ ) {
            for ( int grid_x = 0; grid_x < 100; grid_x++ ) {

                // Second, iterate through the 3D block
                for ( int block_z = 0; block_z < 10; block_z++ ) {
                    for ( int block_y = 0; block_y < 10; block_y++ ) {
                        for ( int block_x = 0; block_x < 10; block_x++ ) {
                            counter += 1;

                            if ( counter % 2 == 0 )
                                number_of_divisible_by_two += 1;
                            if ( counter % 3 == 0 )
                                number_of_divisible_by_three += 1;
                            if ( counter % 5 == 0 )
                                number_of_divisible_by_five += 1;
                        }
                    }
                }

            }
        }
    }

    // Provide our results to the user
    std::cout << std::endl
              << "- Numbers divisible by two: "       << number_of_divisible_by_two       << std::endl
              << "- Numbers divisible by three: "     << number_of_divisible_by_three     << std::endl
              << "- Numbers divisible by five: "      << number_of_divisible_by_five      << std::endl;

    return 0;
}
```
This program does two things - it says hello to the user, and then takes count of the numbers divisible by 2, 3, and 5 from 0 up to 10 billion.

This is done with multiple nested loops - the reason for which will be explained, and the code optimized, in the following chapter on CUDA.

For now, what's apparent and important is that this is a computationally intensive task!

Next, let's build and run this code. By default, Metis users have GCC and G++ (version 11.3.0) preinstalled, which we will now use:
```bash
$ g++ -o hello_world main.cpp
$ ./hello_world
```

The calculation should take 22 seconds, after which we should see our results!

## Getting Started with PBS
We are not currently making full use of Metis with this current setup. What we just ran our code on is called the **login node**, which has nowhere near the amount of computational power that is available to the **compute nodes**, which are where computationally intensive or time-consuming programs should be run.

But how do we do so?

Metis has many users, and each user may have various types of programs, each program with varying hardware requirements. As such, Metis uses a resource manager and job scheduling system by Altair, called [PBS Professional](https://altair.com/pbs-professional). 

In order to make use of this program, we must describe to the system what we need from it, which could be things such as:
- CPU cores
- CPU count
- RAM size
- GPU chunks
- Estimated runtime

...and more.

To do so, we use a PBS script file. For those familiar with systems scripting, this is similar to a `.sh` file on Linux, or a `.bat` file on Windows.

Let's get started by creating a `run.pbs` file with the following contents:
```bash
#!/bin/bash

#PBS -N hello_world
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
PROJECT_DIRECTORY=/home/<your_account_username>/projects/cpp/cpp_on_metis
echo "The job's working directory is $PROJECT_DIRECTORY"
cd $PROJECT_DIRECTORY

# Install GCC
echo ""
echo "Loading GCC..."
module purge; module load gcc/gcc-12.3.0
module list
echo "Done!"

# Compile our code
echo ""
echo "Compiling code..."
g++ main.cpp -o hello_world
echo "Done!"

# Run our binary
echo ""
echo "Executing binary..."
./hello_world
echo "Done!"

# Clean up our binary
rm ./hello_world
```

Before we move on, let's dissect what this does.
```bash
1.  #!/bin/bash
2.
3.  #PBS -N hello_world
4.  #PBS -j oe
5.  
6.  #Note - on Metis
7.  #              Nchunks<=32, for GPU chunks
8.  #              Nchunks<=4096/Ncpus for CPU-only chunks
9.  #              (run 'shownodes' command to find the number of free cpus)
10. #              Ncpus<=128, the total number of CPUs per node is 128
11. #              NPmpi<=Ncpus, the total number of CPUs allocated for MPI tasks,
12. #                              request NPmpi=Ncpus for non-OPENMP jobs
13. #              Ngpus==1,  the total number of GPUs per node is 1
14. #              X<=256,  28 of 32 Metis modes have 256 GB of RAM
15. #                       special jobs can request up to 1024 GB of RAM (4 nodes)
16. #
17. # Below, we request two chunks;
18. #  each chunk needs 8 CPUs, 8 MPI processes, 1 GPU card, and 16 GB RAM
19. #PBS -l select=1:ncpus=1:mpiprocs=1:ngpus=1:mem=2gb
20. #PBS -l walltime=00:15:00
21. 
22. # When to send a status email ("-m abe" sends e-mails at job abort, begin, and end)
23. #--PBS -m ae
24. #--#PBS -M account@niu.edu

...
```

*Lines starting with `#PBS` are not comments, rather, they are PBS-specific commands!*

The following lines are important to understand:
- Line 1 is a [shebang](https://en.wikipedia.org/wiki/Shebang_%28Unix%29) which specifies that the file's commands are to be interpreted by [bash](https://www.gnu.org/software/bash/manual/bash.html).
- Line 3 specifies the name of our file.
- Line 19 specifies the hardware requirements for our job
- Line 20 specifies the estimated runtime of our job
- Lines 23 and 24 specify options for recieveing emails regarding various events

For this job, none of this needs to be modified. The next section, however, will need to be:
```bash
...

26. # Navigate to our working directory
27. PROJECT_DIRECTORY=/home/<your_account_username>/projects/cpp/cpp_on_metis
28. echo "The job's working directory is $PROJECT_DIRECTORY"
29. cd $PROJECT_DIRECTORY

...
```
It's important that we hard-code the exact path on line 27 by replacing `<your_account_username>` with your Metis account's username.

The reason for this only becomes relevant if you have interest in creating non-C++ projects or automating your job submission, so it is worth noting that you can replace `/home/<your_account_username>/projects/cpp/cpp_on_metis` with `$PBS_O_WORKDIR` if you would like. This will be populated with where the job is run from.

Next, we will familiarize ourselves with the `module` commands, which are used on lines 31-36:
```bash
...

31. # Install GCC
32. echo ""
33. echo "Loading GCC..."
34. module purge; module load gcc/gcc-12.3.0
35. module list
36. echo "Done!"

...
```

The `module` commands are somewhat akin to a package manager, allowing you to load packages ("modulefiles") into your environment.

Unlike you, the compute node does not have `gcc` pre-installed. So to make it available to the compute node, we must install it, done in the following fashion:
- Line 34 clears all packages with `module purge`, then installs GCC with `module load gcc/gcc-12.3.0`.
- Line 35 lets you see what's currently installed with `module list`.

This process for installing a package is the same on both the login and compute nodes. To see what packages are available to you, you can run `module av`. To narrow your search by a specific key word, use `module av <keyword>`.

```bash
...

38. # Compile our code
39. echo ""
40. echo "Compiling code..."
41. g++ main.cpp -o hello_world
42. echo "Done!"
43. 
44. # Run our binary
45. echo ""
46. echo "Executing binary..."
47. ./hello_world
48. echo "Done!"
49. 
50. # Clean up our binary
51. rm ./hello_world

...
```

The remaining lines are what you are accustomed to, they use the same build command from before, then run the binary, and finally clean up any artifacts.

## Launching a Job with PBS
We're ready to go! All that's left is to start our job, which can be done easily with the following command:
```bash
$ qsub run.pbs
```

The output will look something like this:
```bash
18681.cm
```

This tells us the ID number of our job. Wait around 30 seconds for the job to finish, and list the contents of the directory!
```bash
$ ls
hello_world.o18681 main.cpp run.pbs
```

Reading the output from our job:
```
$ cat hello_world.o18681
The job's working directory is /home/<your_account_username>/projects/cpp/cpp_on_metis

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

## Closing Thoughts
Congratulations! You've successfully launched your first job on the Metis supercomputer.

This is an impressive achievement. Those who are satisfied with the performance of their programs and are comfortable with only using the C family may even be able to stop here.

However, Metis is capable of much, much more.

In the next chapter, we will discuss utilizing CUDA to weaponize the power of graphics card programming to drastically reduce the computation times of our programs, as well as learning more about the `module` and PBS-related commands.