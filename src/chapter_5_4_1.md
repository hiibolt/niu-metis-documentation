# 5.4.1. PBS Files

This chapter is a summary of the [PBS Professional User Guide](https://www.utrgv.edu/hpcc/_files/documents/pbspro-user-guide.pdf) and the [`qsub` manual](https://www.jlab.org/hpc/PBS/qsub.html), which are fantastic resources if you want to learn more about PBS Professional or `.pbs` files.

Metis has many users, and each user could have a myriad of reqiurements for their application. In order to accommodate each user fairly, and to optimize the execution of jobs, CRCD uses [PBS Professional](https://altair.com/pbs-professional/) by Altair.

Job scripts written for PBS Professional are effectively script files, but alsoe define the parameters and requirements for job execution. These scripts are then run on the batch server, also known as the **compute nodes**.

#### What Does a `.pbs` File Look Like?
```
#!/bin/bash

#PBS -N hello_world
#PBS -j oe
#PBS -l select=1:ncpus=1:mpiprocs=1:ngpus=1:mem=2gb
#PBS -l walltime=00:15:00
#--PBS -m ae
#--PBS -M account@niu.edu

# Navigate to our working directory
PROJECT_DIRECTORY=/home/<your_account_username>/projects/cpp/cpp_on_metis
echo "The job's working directory is $PROJECT_DIRECTORY"
cd $PROJECT_DIRECTORY

# Run our script
./my_binary
```

There are three special properties of a `.pbs` file we should understand!

## PBS Directives

PBS directives are prefaced by `#PBS`, and specify PBS-specific job flags. 

These are actually the exact flags that are effectively passed to the `qsub` command!

For instance, you could specify your email in two different ways:
```pbs
...

#PBS -M you@niu.edu

...
```

...or, with the command flag: `qsub -M you@niu.edu`.

To comment out a PBS directive, replace `#PBS` with `#--PBS`.

*It's worth noting that providing a flag by CLI will override the directives in the PBS file, which can be helpful if you want default values that can (optionally!) be specified differently at time of submission.*

**Common Directives**:
* `#PBS -N <name>` or `qsub -N <name>`

    This will specify the name of the job in the PBS system, emails, and file output.

    Example Usage:
    ```bash
    $ qsub -N hello_world run.pbs
    20000.cm
    $ ls
    hello_world.o20000
    ```
* `#PBS -l <resource_1>:...:<resource_n>` or `qsub -l <resource_1>,...,<resource_n>`

    Specifies the resources that the job needs. You can find more about the types to specify in the [PBS User's Guide](https://www.utrgv.edu/hpcc/_files/documents/pbspro-user-guide.pdf), but the template that CRCD provides is very ideal.
    
    **Note - on Metis**:
    * **Nchunks<=32**, for GPU chunks
    * **Nchunks<=4096/Ncpus** for CPU-only chunks

        (run 'shownodes' command to find the number of free cpus)
    * **Ncpus<=128**, the total number of CPUs per node is 128
    * **NPmpi<=Ncpus**, the total number of CPUs allocated for MPI tasks,

        request NPmpi=Ncpus for non-OPENMP jobs
    * **Ngpus==1**,  the total number of GPUs per node is 1
    * **X<=256**,  28 of 32 Metis modes have 256 GB of RAM

        special jobs can request up to 1024 GB of RAM (4 nodes)

    Below, we request two chunks; each chunk needs 8 CPUs, 8 MPI processes, 1 GPU card, and 251 GB RAM, and we expect the total job runtime (walltime) to be 15 minutes.

    If you are requesting a GPU, you are reserving *an entire* node. Accordingly, you should use the entire capacity of RAM available to said node (251GB). Some nodes also have 1259GB available by special request.

    Example (`run.pbs` file):
    ```pbs
    #PBS -l select=1:ncpus=8:mpiprocs=8:ngpus=1:mem=2gb
    #PBS -l walltime=00:15:00
    ```

    To learn how to optimize these values, see the [official Metis protocol](https://www.niu.edu/crcd/current-users/getting-started/queue-commands-job-management.shtml#jobcontrol).
* `#PBS -j <n | oe>` or `qsub -j <n | oe>`

    Specifies whether the standard error stream should be merged with the standard output stream.

    Specifying `oe` means that both `stderr` and `stdout` will be in the same output file.

    Specifying `n`, or not specifying at all, means they will be in different files.

    ```bash
    $ qsub -j n run.pbs
    20000.cm
    $ ls
    hello_world.o20000
    hello_world.e20000
    ```
* `#PBS -m <n | a*b*e*>` or `qsub -m <n | a*b*e*>`

    Specifies when mail about your job should be sent, with the following key:
    * To send mail when it aborts, add `a`
    * To send mail when it begins, add `b`
    * To send mail when it ends, add `e`
    * To not send mail, specify `n` or do not use this directive.
* `#PBS -M <email>` or `qsub -M <email>`

    Specifies the email any job alert emails should be sent to.

    Email should only ever be sent to NIU-based emails for additional security.

## PBS Environment Variables

There are two types of environment variables. Those which are prefaced by `PBS_O_` are influenced by the job's originating environment (the user environment!). Those which are prefaced by `PBS_` are provided by PBS.

All examples are from a `.pbs` file, as these environment variables are only populated inside a batch job.

**Common Environment Variables**:
* `TMPDIR`

    This is one of the most important directories, as it's deleted when the job finishes.

    Any build artifacts, unimportant files, or other ephemeral content should be stored in this directory - this will make your job much cleaner!

    Example PBS Usage and Output:
    ```pbs
    echo "This job's temporary directory is: '$TMPDIR'"
    This job's temporary directory is '/scratch/pbs.20000.cm'
    ```
* `PBS_O_HOME`

    The home folder of the user running the command.

    Example PBS Usage and Output:
    ```pbs
    echo "My home directory is: '$PBS_O_HOME'"
    My home directory is: '/home/you'
    ```
* `PBS_O_LOGNAME`

    The username of the invoking user.

    Example PBS Usage and Output:
    ```pbs
    echo "My login username is: '$PBS_O_LOGNAME'"
    My login username is 'you'
    ```
* `PBS_O_PATH`

    The PATH environment variable from the invoking user.

    Example PBS Usage and Output:
    ```pbs
    echo "My path is: '$PBS_O_PATH'"
    My path is: '/urs/new/bin:/usr/local/bin:/bin'
    ```
* `PBS_O_SHELL`

    The shell of the invoking user.

    Example PBS Usage and Output:
    ```pbs
    echo "My shell path is: '$PBS_O_SHELL'"
    My shell path is: '/sbin/csh'
    ```
* `PBS_O_HOST`

    The machine hostname of the server.

    Example PBS and Output:
    ```pbs
    echo "The machine hostname: '$PBS_O_HOST'"
    The machine hostname: 'metis'
    ```
* `PBS_O_WORKDIR`

    The user's working directory (at time of invocation).

    Example PBS and Output:
    ```pbs
    echo "My current working directory: '$PBS_O_WORKDIR'"
    My current working directory: '/home/you'
    ```
* `PBS_JOBID`

    The ID of the batch job.

    Example PBS Usage and Output:
    ```pbs
    echo "The ID of this job: '$PBS_JOBID'"
    The ID of this job: '16386.cm'
    ```

### The Shebang
At the top of the script, we can see the [shebang](https://en.wikipedia.org/wiki/Shebang_%28Unix%29).

A shebang specifier which interpreter PBS should use. In almost every case, it's best to use [Bash](https://www.gnu.org/software/bash/), which is located at `/bin/bash`.

*This line is reqiured, but likely doesn't need to be modified!*
