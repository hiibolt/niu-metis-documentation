# 4.3. Creating your Own Docker Images
<small>*Associated CRCD Documentation: [PBS](https://crcd.niu.edu/crcd/current-users/getting-started/run-interactive-jobs.shtml)*</small>

*You can find the code mentioned in this chapter [in this book's repository](https://github.com/hiibolt/niu-metis-documentation/tree/main/projects/docker/custom_image)!*

Unlike previous chapters, this will not have an example project, and will instead be more free-form to act as a basepoint for your own research!

We will discuss some possible venues from where to learn Dockerfile syntax, building images, and running them on Metis to create a solution that fits your quota.

## Goals
* Look at some examples of a `Dockerfile`
* Get a rough idea for how to write your own `Dockerfile`
* Get a rough idea on resources about publishing your own custom Docker Images

## What Actually Is a Docker Image?
In the past, we've only used images from the [Docker Hub](https://hub.docker.com). But how are those images created?

Docker Images are defined and built from a [`Dockerfile`](https://docs.docker.com/reference/dockerfile/). 

They are somewhat similar in nature to PBS files, but they define a lot more, and allow elevated permissions plus more granular control.

Defined below is a `Dockerfile` for a Python project, which is thoroughly documented:
```bash
# syntax=docker/dockerfile:1

# This specifies the base image to base FROM for the image that will be built.
# 
# In this case, we are using the official Python image from Docker Hub.
#
# The tag `3.12.5-bookworm` specifies the version of the Python image to use.
# The tag `bookworm` is a codename for the version of Debian that the image is based on.
# The tag `3.12.5` is the version of Python that the image has preloaded.
# 
# To find more base images, visit `https://hub.docker.com/`!
FROM python:3.12.5-bookworm

# Create a directory at /app to store the application code inside the image.
WORKDIR /app

# RUN instructions are executed during the build process of the image.
# 
# This means, once the image is built, the following commands will be executed,
#  but not when the container is run. For instance, the following commands will
#  be executed when the image is built, but not when the container is run:
#  - `apt update` (updates the package manager)
#  - `apt install -y cmake build-essential`
#  - `python -m venv .venv` (creates a virtual environment)
#  - `.venv/bin/pip install numpy` (installs the numpy package)
#
# These RUN commands are extremely useful for setting up the environment, particularly
#  for packages like `numpy` that require compilation with `cmake` and `build-essential`.
#
# It's worth noting that the Docker build process is not interactive, so you can't
#  interact with the terminal during the build process. This is why the `-y` flag is
#  used in the `apt install` command to automatically answer "yes" to the prompt!
RUN apt update
RUN apt install -y cmake build-essential

RUN python -m venv .venv
RUN .venv/bin/pip install numpy

# COPY the source code from
#  the host machine (`.`, where the Dockerfile is located)
#  to the image     (`.`, or the working directory).
#
# As specified in the `WORKDIR` instruction above, the working
#  directory is `/app`.
#
# For example, running `docker build ...` from the directory of this project
#  will copy from `/home/user/projects/docker/premade_image/main.py` to `/app/main.py`
#  in the image!
COPY . .

# When the application is built, the container will run the following CMD.
#
# The CMD instruction specifies the command that will be executed when the container
#  is run, but not when the image is built. For instance, the following command will
#  be executed when the container is run:
#  - `.venv/bin/python3 main.py` (runs the `main.py` script)
#
# In this case, the command is `.vent/bin/python3 main.py`, which will run the `main.py` script.
CMD .venv/bin/python3 main.py
```

`Dockerfiles` live in the root of a project. An example Python project layout:
```
src/
- main.py
- Dockerfile
```

The reason why `Dockerfiles` are useful becomes more apparent the more complex and dependency-heavy your project is. Each command in a `Dockerfile` is cached step-by-step, which means, after the first time the above `Dockerfile` is built, steps such as dependency installation with `apt` are not performed again.

This means that builds with `Dockerfile` are exceptionally fast, if properly optimized!

Linked [here](https://www.digitalocean.com/community/tutorials/how-to-optimize-docker-images-for-production) is a fan-favorite crash course in optimizing `Dockerfiles`.

## How Do I Write a `Dockerfile` From the Ground Up?
This varies from project-to-project based on decisions such as:
* Base operating system
* Programming Language
* Dependencies
* Whether you plan to use CUDA or CUDNN

From the get-go, if you plan to use CUDA and/or CUDNN, you should use [NVIDIA's base images](https://hub.docker.com/r/nvidia/cuda/) in your `FROM` instructions. This will save you a ton of time with configuration, as it's much simpler to install a programming language than to install CUDNN or CUDA.

Depending on your project, Docker has wonderful guides linked [here](https://docs.docker.com/language/). These include:
- Go
- Python
- R
- Rust

...and many more.

Once you have written and built your image, you should test it locally on your own machine. In fact, all Docker development is best done on your local machine.

## Publishing your Image to a Public Registry
Now, unfortunately, I have not found a way to build Docker Images on a login node on Metis in a way that allows you to copy the image over to the desired compute node.

The workaround is to build them locally, publish our images, and then pull them onto the compute node.

## How Do I Choose Where to Publish?

There are two good options for public registries:
- Docker Hub
- GitHub Container Repository (GHCR)

If you are not tracking your project with GitHub already, I suggest that you follow [this guide](https://www.geeksforgeeks.org/docker-publishing-images-to-docker-hub/) to publish to Docker Hub (what we have used in past chapters).

If you are tracking with GitHub, it may be more convenient to instead use GitHub Actions to automatically build and publish your image with each commit. 

GitHub Actions is significantly more ideal, but does build slower. Our team chose to use this route, since our entire codebase is on GitHub! Linked below is documentation on how to do so, and the two repositories we have automatic builds enabled on.
* [GitHub's Documentation](https://docs.docker.com/build/ci/github-actions/)
* [`igait-openpose`](https://github.com/igait-niu/igait-openpose) (runs on Metis)
* [`igait-backend`](https://github.com/igait-niu/igait-backend) (runs on AWS)

With this approach, you can containerize virtually any project with ease.

## Our Team's Usage
The [iGAIT research team](https://github.com/igait-niu) found great success using Metis to accelerate our workflow.

The primary chokepoint of our workflow was [OpenPose](https://github.com/CMU-Perceptual-Computing-Lab/openpose), which we use to create pose mappings of a human.

<img src="https://github.com/CMU-Perceptual-Computing-Lab/openpose/raw/master/.github/media/pose_face_hands.gif"></img>

Previously, on AWS and tested locally, runtime was upwards of 3 hours - and occupied the entirety of the available resources.

However, on Metis, on the login nodes - that time dropped down, but not as far as we wanted it.

Original inference times (login node, with GPU, Docker with NVIDIA CDI):
* Total: 1 hour+ total, job killed for long runtime
* Video 1: **43 minutes**
* Video 2: **17 minutes** (did not finish)

New inference times (compute node, with GPU, Docker with NVIDIA CDI):
* Total: <1 minute :D
* Video 1: **18.689683 seconds**
* Video 2: **24.962916 seconds**

What is very interesting is that our job had very minimal hardware specifications - you don't always need heavy CPU core counts if the GPU can handle it.
```
#PBS -l select=1:ncpus=8:mpiprocs=1:ngpus=1:mem=251gb
```

*Note: Although the 2GB is the most effecient amount we found, it is pointless as reserving a GPU also reserves the entire node*.

You can find our Dockerfiles [here](https://github.com/igait-niu/igait-openpose/tree/main). There are multiple versions, the simplest being the CPU-only build.