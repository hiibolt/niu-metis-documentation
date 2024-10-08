# 4.2. Using GPU Acceleration With Docker
<small>*Associated CRCD Documentation: [PBS](https://crcd.niu.edu/crcd/current-users/getting-started/run-interactive-jobs.shtml)*</small>

*You can find the code mentioned in this chapter [in this book's repository](https://github.com/hiibolt/niu-metis-documentation/tree/main/projects/docker/premade_image_gpu)!*

Now we must address how to use GPU passthrough on Metis with Podman (Docker), which can quickly elevate our programs to higher performance with the power of GPU acceleration!

## Goals
- Pass through a GPU to Podman

## The Problem(s)
In order to do so, we must solve the following problems:
* **1** - Our GPUs are not passed through to Podman (Docker) by default
* **2** - NVIDIA drivers and the CUDA runtime are not installed on most Docker Images
* **3** - NVIDIA device files aren't always loaded on the compute node

## The Solution(s)
### 1 - Our GPUs are not passed through to Podman (Docker) by default
To solve this, we add two flags to our `/bin/podman` command:
```bash
$ /bin/podman run ...               \
    --device nvidia.com/gpu=all   \
    --security-opt=label=disable  \
    some/image
```
This will ensure that the GPU is passed through to our Docker Container.
### 2 - NVIDIA drivers and the CUDA runtime are not installed on most Docker Images
CUDA drivers are notoriously difficult to install, so it's highly recommended to use a base image that already has them pre-installed.

For the purpose of this example, we will be using NVIDIA's [base image](https://hub.docker.com/r/nvidia/cuda/), which has CUDA pre-installed.
### 3 - NVIDIA device files aren't always loaded on the compute node
Occasionally, the `/dev` files for the NVIDIA GPUs disappear on compute nodes.

To solve this, we use a relatively hacky but functional solution - running a CUDA-based binary to force them to load.

For the sake of demonstration, we'll use the binary we developed in **Chapter 2.2**!

## Implementation
First, let's create our project directory as we have in previous projects:
```bash
$ mkdir /lstr/sahara/<your_project>/<you>
$ mkdir /lstr/sahara/<your_project>/<you>/docker
$ mkdir /lstr/sahara/<your_project>/<you>/docker/premade_image_gpu
$ cd /lstr/sahara/<your_project>/<you>/docker/premade_image_gpu
```

Next, we need a binary that forces CUDA to load. We'll build the project from Chapter 2.1 and have it output here:
```bash
$ module purge
$ module load cuda/cuda-11.8
$ nvcc -o initialize_cuda /lstr/sahara/<your_project>/<you>/cuda/cuda_on_metis/main.cu
```

Finally, we'll implement everything mentioned above.

Create a `run.pbs` file with the following contents:
```sh
#!/bin/bash

#PBS -N premade_image_gpu
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
#PBS -l select=1:ncpus=8:mpiprocs=1:ngpus=1:mem=251gb
#PBS -l walltime=00:15:00

# When to send a status email ("-m abe" sends e-mails at job abort, begin, and end)
#--PBS -m ae
#--#PBS -M account@niu.edu

PROJECT_DIRECTORY=/lstr/sahara/<your_project>/<you>/docker/premade_image_gpu
echo "This job's working directory is $PROJECT_DIRECTORY"
cd $PROJECT_DIRECTORY

# Enable linger for the user
echo ""
echo "Enabling linger for the user..."
loginctl enable-linger <your_account_username>
echo "Done!"

# Initialize GPU device files by running our script with CUDA
echo ""
echo "Running a quick CUDA program..."
module purge; module load cuda/cuda-11.8
./initialize_cuda \
    > /dev/null 2>&1 # You can remove this line if you want to see the logs!
echo "Done!"

# Start the container
# 
# There are five flags, most of which will never change:
# - `-v $PROJECT_DIRECTORY:/home` mounts the project directory to the `/home` 
#    directory in the container.
# - `-w /home` sets the working directory in the container to `/home`.
# - `-t` allocates a pseudo-TTY. This is useful for running the container in
#    the background.
# - `-d` runs the container in the background.
#
# The last argument is the image name. This is the only thing that will change
#  between projects, this is the name of the image we want to run.
# 
# For instance, in this case, `cuda:12.6.0-cudnn-runtime-ubuntu22.04`:
# - `cuda` is the name of the image.
# - `12.6.0-cudnn-runtime-ubuntu22.04` is the tag of the image, which specifies
#    the version of the image, the base operating system, and any additional
#    software that is included in the image.
#
# Millions of pre-built images are available on Docker Hub, and will likely 
#  already have an image that suits your needs! You can search for images here:
#  https://hub.docker.com/
#
# Note: There may be many logs that are printed to the console when the container
#  is started. Despite being error-level, this is normal, and you can ignore them.
echo ""
echo "Starting the container..."
/bin/podman run                                 \
    -v $PROJECT_DIRECTORY:/home                 \
    -w /home                                    \
    --name cuda_container                       \
    --device nvidia.com/gpu=all                 \
    --security-opt=label=disable                \
    -t -d                                       \
    nvidia/cuda:12.6.0-cudnn-devel-ubuntu20.04  \
    #> /dev/null 2>&1 # You can remove this line if you want to see the logs!
echo "Done!"

# Run our `nvidia-smi` command
#
# The `exec` command runs a command in a running container. In this case, we are
#  running the `nvidia-smi` command in the `cuda_container` container.
# 
# There is a generic error message, which can be ignored.
echo ""
echo "Running the \`nvidia-smi\` command..."
/bin/podman exec cuda_container nvidia-smi
echo "Done!"

# Kill the container
#
# The `kill` command stops a running container. In this case, we are stopping the
#  `cuda_container` container.
echo ""
echo "Stopping the container..."
/bin/podman kill cuda_container \
    > /dev/null 2>&1 # You can remove this line if you want to see the logs!
echo "Done!"

# Remove the container
#
# The `rm` command removes a container. In this case, we are removing the
#  `cuda_container` container.
echo ""
echo "Removing the container..."
/bin/podman rm cuda_container \
    > /dev/null 2>&1 # You can remove this line if you want to see the logs!
echo "Done!"
```

As always, don't forget to replace occurrences of `<your_project>` and `<you>` with your actual Metis username.

Now, let's discuss what's changed from Chapter 3.1.

Firstly, we ensure CUDA `/dev` files are created:
```bash
...

# Initialize GPU device files by running our script with CUDA
echo ""
echo "Running a quick CUDA program..."
module purge; module load cuda/cuda-11.8
./initialize_cuda \
    > /dev/null 2>&1 # You can remove this line if you want to see the logs!
echo "Done!"

...
```

Secondly, we add the flags which make our GPU visible to Podman (Docker), and we use NVIDIA's CUDA base image:
```bash
...

# Start the container
# 
# There are five flags, most of which will never change:
# - `-v $PROJECT_DIRECTORY:/home` mounts the project directory to the `/home` 
#    directory in the container.
# - `-w /home` sets the working directory in the container to `/home`.
# - `-t` allocates a pseudo-TTY. This is useful for running the container in
#    the background.
# - `-d` runs the container in the background.
#
# The last argument is the image name. This is the only thing that will change
#  between projects, this is the name of the image we want to run.
# 
# For instance, in this case, `cuda:12.6.0-cudnn-runtime-ubuntu22.04`:
# - `cuda` is the name of the image.
# - `12.6.0-cudnn-runtime-ubuntu22.04` is the tag of the image, which specifies
#    the version of the image, the base operating system, and any additional
#    software that is included in the image.
#
# Millions of pre-built images are available on Docker Hub, and will likely 
#  already have an image that suits your needs! You can search for images here:
#  https://hub.docker.com/
#
# Note: There may be many logs that are printed to the console when the container
#  is started. Despite being error-level, this is normal, and you can ignore them.
/bin/podman run                                 \
    -v $PROJECT_DIRECTORY:/home                 \
    -w /home                                    \
    --name cuda_container                       \
    --device nvidia.com/gpu=all                 \
    --security-opt=label=disable                \
    -t -d                                       \
    nvidia/cuda:12.6.0-cudnn-devel-ubuntu20.04  \
    #> /dev/null 2>&1 # You can remove this line if you want to see the logs!

...
```

Third and finally, to test, we run `nvidia-smi`, which details available NVIDIA GPUS:
```bash
...

# Run our `nvidia-smi` command
#
# The `exec` command runs a command in a running container. In this case, we are
#  running the `nvidia-smi` command in the `cuda_container` container.
# 
# There is a generic error message, which can be ignored.
echo ""
echo "Running the \`nvidia-smi\` command..."
/bin/podman exec cuda_container nvidia-smi
echo "Done!"

...
```

Finally, it's worth noting that the first execution will take some time - the NVIDIA CUDA image is quite large at ~5GB. To test our PBS job:
```bash
$ qsub run.pbs
18731.cm
```

After some time (remember, you can check the status of a job with `qstat -x <job_id>`!):
```
$ cat premade_image_gpu.o18731
...

Fri Aug 16 21:50:56 2024       
+---------------------------------------------------------------------------------------+
| NVIDIA-SMI 530.30.02              Driver Version: 530.30.02    CUDA Version: 12.1     |
|-----------------------------------------+----------------------+----------------------+
| GPU  Name                  Persistence-M| Bus-Id        Disp.A | Volatile Uncorr. ECC |
| Fan  Temp  Perf            Pwr:Usage/Cap|         Memory-Usage | GPU-Util  Compute M. |
|                                         |                      |               MIG M. |
|=========================================+======================+======================|
|   0  NVIDIA A100-PCIE-40GB           On | 00000000:27:00.0 Off |                    0 |
| N/A   38C    P0               40W / 250W|      0MiB / 40960MiB |      0%      Default |
|                                         |                      |             Disabled |
+-----------------------------------------+----------------------+----------------------+
                                                                                         
+---------------------------------------------------------------------------------------+
| Processes:                                                                            |
|  GPU   GI   CI        PID   Type   Process name                            GPU Memory |
|        ID   ID                                                             Usage      |
|=======================================================================================|
|  No running processes found                                                           |
+---------------------------------------------------------------------------------------+

...
```

## Closing Thoughts

Congratulations! You've officially achieved full GPU passthrough to Podman (Docker) through the PBS job scheduling system!

This is quite the technical feat, and displays some of the most impressive containerization and supercomputing technologies available.

Almost every conceivable project can be run on Metis using this technique, from CUDA-based quantum simulations, to machine learning models, to facial recognition software.

For those whos' projects are complete using this tactic, you can safely stop reading here, if you would like. If SSH automation (**Chapter 4.1**) interests you, you can also safely skip to that chapter.

The next chapter, **Chapter 3.3**, will provide insight into writing your own base images from the ground up, and some tactics for optimizing base images for build-time and size.
