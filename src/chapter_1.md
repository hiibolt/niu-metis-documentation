# 1. Introduction
Welcome! This book serves as an all-in-one crash course in utilizing Metis.

It aims to allow anyone from any discipline, regardless of Linux experience, to get started. As such, no knowledge of any language is required to follow any example, as the focus of the examples is instead to help you understand Metis.

On top of this, should a step confuse you, the final product of every project example in this book can also be found in this book's [repository](https://github.com/hiibolt/niu-metis-documentation/tree/main/projects).

## Primary Purpose
The goal of this book is to allow faculty at NIU to hit the ground running with their work. 

We want you to focus on completing your work - not getting the hardware to work.

This book will teach you the skills to help focus on writing your application from the ground up.

### Explored Use Cases
There are five use cases covered here, with increasing levels of control over Metis:
* **Chapter 2.1 - Running a C++ project**
    - No additional configuration
    - PBS only
* **Chapter 2.2 - Running a CUDA project**
    - Loading CUDA via the `module` command
    - PBS only
* **Chapter 3.1 - Running a language not installed on Metis, such as Python 3.11**
    - Downloading a pre-built Docker Image with `python` version 3.11 installed
    - PBS with Docker via Podman
* **Chapter 3.2 - Running packages not installed on Metis with GPU passthrough**
    - Downloading a pre-built Docker Image
    - Passing through GPUs to Docker
    - PBS with Docker and NVIDIA Container Toolkit via Podman
* **Chapter 3.3 - Running virtually any project using custom Docker Images**
    - Writing, building, and publishing your own Docker Image
    - Passing through GPUs to Docker
    - PBS with Docker and NVIDIA Container Toolkit via Podman
* **Chapter 4.1 - SSH Automation**
    - Demonstrates programmatic submission of PBS jobs via SSH for the purpose of fitting Metis into existing systems.

## Where Do I Need to Read to?
### Cases Where Docker May Not Be Needed
IF your application is either of the following, you shouldn't or can't use Docker.
- Native C, Go, or Python applications *with pre-installed or no dependencies*
- OpenMPI-based applications

If it's one of those two, chapters **2.1** and **2.2** will be of great use!

The following chapters may not be as useful, as they touch primarily on Docker.
### Cases Where Docker Is Needed
If your application is any of the following, it's highly recommended to use Docker: 
- Applications with a language not listed above
- Applications with dependencies Metis does not have encapsulated in its modulefiles
- Applications with complex or circular dependencies
- Applications which require a different operating system

If you only need CPU-based computation, chapters **2-3.1** will teach you everything you need.

If you need GPU passthrough or have a complicated project, it is recommended to read this book in its entirety!