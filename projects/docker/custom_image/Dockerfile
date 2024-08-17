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