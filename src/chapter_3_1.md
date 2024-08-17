# 3.1. Using Pre-Made Docker Images
*You can find the code mentioned in this chapter [in this book's repository](https://github.com/hiibolt/niu-metis-documentation/tree/main/projects/docker/premade_image)!*

We will first begin by using a language which is *not* among the modules which Metis provides, Python 3.11.

In actuality, Metis does offer Python, as seen below:
```
$ module av python
-------------------- /etc/modulefiles ---------------------
python/python-3.9.10  python/python-3.12.4
```

...but, at the time of writing this, it does not have Python 3.11, which is among the most commonly used versions.

So, how do we fix this?

Well, we ourselves can't fix the global modulefiles on Metis, which means under normal circumstances, we would have to reach out to Metis staff to have the module fixed - something that takes away from both your own time and the time of the Metis staff.

You are able to define your own modulefiles, but this is a time consuming task, and it can't solve everything.

## Goals
- Learn how to use Podman and Docker
- Learn how to install dependencies via Podman's CLI
- Learn how to use Podman in a PBS script file
- Learn how to kill Podman to avoid uptime emails and alerts

## The Problem
Modulefiles struggle or are outright impossible to create with any of the following cases:
* Packages which can only run on certain operating systems, and specific versions of those operating systems
* Packages which have dense dependency trees
* Packages which have circular dependencies
* Packages which need elevated permissions
* Packages with long build times, where a distributed binary may be preferred
* Closed-source or unfree packages (which are very common in machine learning!)
* Huge numbers of dependencies

This isn't to say it's impossible to manually build every single dependency for your project, and also include them manually.

However, this is an **extremely** time-consuming process, and time spent doing this will only take away from your core work.

Dependency installation should be a matter of lines, not weeks.

## The Solution
[Docker](https://www.docker.com/), an extremely powerful containerization and encapsulation tool that allows developers to define virtual machines with a level of granularity rarely found in modern computing. Docker allows you to select an operation system as a base, install packages and libraries, and define run behaviour.

We will be using an overlay on Docker called [Podman](https://podman.io/). It allows us to use Docker containers despite not having elevated permissions on Metis. Understanding of Podman isn't required - all Docker commands can have `docker` replaced with `podman` (or in our case, `/bin/podman`).

If you haven't already, create your projects directory, a new directory for Docker projects, and finally a directory for this project:
```bash
$ mkdir ~/projects
$ mkdir ~/projects/docker
$ mkdir ~/projects/docker/premade_image
$ cd ~/projects/docker/premade_image
```

Next, let's create a `main.py` file with the following contents:
```python
print( "Hello, Metis!" )
```

Now, how do we get Docker to run this file?

For your own projects, you can search the [Docker Hub](https://hub.docker.com/) for programming languages, software, and more. You can also use a base image like `ubuntu:22.04` or `debian:bookworm`, which contain nothing but the operating system with no additional packages or programming languages.

From there, you can use the `exec` command to install the languages or packages with that operating system's respective package manager. We will go over the usage of the `exec` command with examples shortly!

We'll start by downloading and running a [Docker Image](https://docs.docker.com/guides/docker-concepts/the-basics/what-is-an-image/), which will be built on the Debian operating system version 12.6 "Bookworm", and include Python 3.11.9:
```
$ /bin/podman run             \
    -v ./.:/home            \
    -w /home                \
    --name python_container \
    -t -d                   \
    python:3.12.5-bookworm
WARN[0000] Network file system detected as backing store.  Enforcing overlay option `force_mask="700"`.  Add it to storage.conf to silence this warning 
f258979e09d0923ebb815b0b0baae9ae9cb2de18ace02a4aa282920c673073d9
```

The first line with the warning can be safely ignored. It's likely that by the time you are reading this, it's been silenced.

Next, let's run our Python script!
```
$ /bin/podman exec python_container python3 main.py
...
Hello, World!
```

Congratulations! You've just run a version of Python that's not installed on Metis at all. But, what if our Python script needed some dependencies?

Overwrite the `main.py` file with the following contents:
```python
import numpy as np

print( "Hello, Metis!" )
```

If we try to run our script again, we get an error:
```
$ /bin/podman exec python_container python3 main.py
...
Traceback (most recent call last):
  File "/home/main.py", line 1, in <module>
    import numpy as np
ModuleNotFoundError: No module named 'numpy'
```

Let's create our [Python virtual environment](https://docs.python.org/3/library/venv.html), and install `numpy` using the `exec` command! Run the following:
```bash
$ /bin/podman exec python_container python -m venv .venv
$ /bin/podman exec python_container .venv/bin/pip install numpy
```

Running our script again:
```
$ /bin/podman exec python_container .venv/bin/python3 main.py
...
Hello, Metis!
```

Nicely done! Lastly, let's kill and remove our container:
```bash
$ /bin/podman kill python_container
$ /bin/podman rm python_container
```

Again, congratulations! You've successfully downloaded a Docker Image, installed some dependancies, and run them on the login node!

## Docker in PBS
Now, we just ran that Docker image on the *login node*, not the compute nodes. So how do we write a PBS file to automate what we just did?

Create a `run.pbs` file with the following contents:
```bash
#!/bin/bash

#PBS -N premade_image
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

PROJECT_DIRECTORY=/home/<your_account_name>/projects/docker/premade_image
echo "This job's working directory is $PROJECT_DIRECTORY"
cd $PROJECT_DIRECTORY

# Enable linger for the user
echo ""
echo "Enabling linger for the user..."
loginctl enable-linger <your_account_name>
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
# For instance, in this case, we are running the `python:3.12.5-bookworm` image:
# - `python` is the name of the image.
# - `3.12.5-bookworm` is the tag of the image, which specifies the version of the
#    image we want to run.
#
# Millions of pre-built images are available on Docker Hub, and will likely 
#  already have an image that suits your needs! You can search for images here:
#  https://hub.docker.com/
#
# Note: There may be many logs that are printed to the console when the container
#  is started. Despite being error-level, this is normal, and you can ignore them.
echo ""
echo "Starting the container..."
/bin/podman run                  \
    -v $PROJECT_DIRECTORY:/home  \
    -w /home                     \
    --name python_container      \
    -t -d                        \
    python:3.12.5-bookworm       \
    > /dev/null 2>&1 # You can remove this line if you want to see the logs!
echo "Done!"

# Run our python script
#
# The `exec` command runs a command in a running container. In this case, we are
#  running the `python3 main.py` command in the `python_container` container.
# 
# There is a generic error message, which can be ignored.
echo ""
echo "Running the python script..."
/bin/podman exec python_container .venv/bin/python3 main.py
echo "Done!"

# Kill the container
#
# The `kill` command stops a running container. In this case, we are stopping the
#  `python_container` container.
echo ""
echo "Stopping the container..."
/bin/podman kill python_container \
    > /dev/null 2>&1 # You can remove this line if you want to see the logs!
echo "Done!"

# Remove the container
#
# The `rm` command removes a container. In this case, we are removing the
#  `python_container` container.
echo ""
echo "Removing the container..."
/bin/podman rm python_container \
    > /dev/null 2>&1 # You can remove this line if you want to see the logs!
echo "Done!"
```

This is largly the same, and only two things need to be modified to fit your Metis account:
```
...

PROJECT_DIRECTORY=/home/<your_account_name>/projects/docker/premade_image
echo "This job's working directory is $PROJECT_DIRECTORY"
cd $PROJECT_DIRECTORY

# Enable linger for the user
echo ""
echo "Enabling linger for the user..."
loginctl enable-linger <your_account_name>
echo "Done!"

...
```
Be sure to replace the two `<your_account_name>` instances with your own account! The linger command is unique to Podman (Docker) jobs in PBS, and ensures it has the nessecary permissions to run your jobs.

With that, let's test our job!
```
$ qsub run.pbs
18712.cm
$ cat premade_image.o18712
This job's working directory is /home/<your_account_name>/projects/docker/premade_image

Enabling linger for the user...
Done!

Starting the container...
Done!

Running the python script...
time="2024-08-16T14:57:08-05:00" level=warning msg="Network file system detected as backing store.  Enforcing overlay option `force_mask=\"700\"`.  Add it to storage.conf to silence this warning"
Error: can only create exec sessions on running containers: container state improper
Done!

Stopping the container...
Done!

Removing the container...
Done!
```

Lastly, we must kill off our Podman processes on the login node, or else we'll recieve emails about extended uptime. 

There are many, so it's easier to kill instead everything under your username. This will close your shell connection, so please save any unfinished work before doing so. 

This will cause additional load times next time you login to Metis (10-20 seconds), but is important to do.
```bash
pkill -U <your_account_username>
```

## Closing Thoughts
Congratulations! You now have the skills needed to tackle most CPU-only applications.

You can modify the base image to fit the operating system, languages, and software you need! You can also add or modify `exec` commands to install more languages, libraries, or software to be able to load anything else your software might need.

If you'd like to learn more about the `run`, `exec`, `kill`, or `rm` commands, additional documentation can be found in the **Conclusion and Helpful Resources** chapter!

If your application does not make use of the GPU, and you have no interest in automation or integration, you likely don't need to read any further. If you do, then feel free to continue onto **Chapter 3.2 - Using GPU Acceleration with Docker**!
