# 2. Basic Metis Usage
This first chapter will provide two into-the-fire projects that will teach you the core systems of Metis. This will be done through a simple C++ project, followed by an optimized version written with CUDA.

Reminder - you don't need to know Bash (Linux Commands), C++, or CUDA to follow along! It may help your understanding to know some basic Bash commands, so feel free to quickly glance over **Chapter 5.1 - Bash**.

These next two chapters lay the foundational skills needed to use the advanced techniques in the following chapters, and it is highly recommended that you read them before proceeding!

## Overview of the Chapters
### Chapter 2.1: C++ on Metis
* **Goals**: Familiarize with basic commands and job submission on Metis.
* **C++ Boilerplate**: Create and run a basic "Hello, World" C++ program with computational loops.
* **PBS Basics**: Write a PBS job script to run your C++ program on compute nodes.
* **Execution**: Compile and run the C++ program locally and via PBS.
* **Outcome**: You will be able to understand job submission, the PBS script structure, and basic module commands.
### Chapter 2.2: Building a CUDA Project from the Ground Up
* **Goals**: Learn to use CUDA for GPU programming on Metis.
* **CUDA Boilerplate**: Write a CUDA program to achieve the same task as in Chapter 1.1 but using GPU acceleration.
* **CUDA Modules**: Install and use the CUDA compiler (nvcc) with module commands.
* **Execution**: Compile and run your CUDA program, observing performance improvements.
* **PBS for CUDA**: Adapt the PBS script to load CUDA modules and compile with nvcc.
* **Outcome**: You will be able to leverage CUDA for faster computation and understand the structure of both CUDA programs and PBS scripts.