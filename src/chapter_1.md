# 1. Introduction
Welcome! This book serves as an all-in-one crash course in utilizing Metis.

Metis (commissioned in September 2023) is a 32-node CPU/GPU hybrid cluster running Red Hat Enterprise Linux 8.x operating system. 1PB of shared disk is provided by a Cray ClusterStor E1000 storage server. Each compute node is an HPE DL 385 Gen 10+ V2 server equipped with:
* 2x AMD EPYC 7713 CPUs 2.0 GHz 64-core processors
* 251-1259 GB RAM, 1 x 4TB SSD scratch disk drives
* 1 x NVIDIA A100 GPU, Amper™ architecture, 40 GB RAM each card
* All 32 nodes are connected via a 200 Gbps Infiniband network

To learn more about Metis, you can see the [METIS layout and specification](https://crcd.niu.edu/crcd/images/metislayoutandspecification.pdf).

## Primary Purpose
The goal of this book is to allow faculty at NIU to hit the ground running with their research. 

We want you to focus on completing your work - not getting the hardware to work.

This book will teach you the skills to help focus on writing your application from the ground up.

Additionally, should a step confuse you, the final product of every project example in this book can also be found in this book's [repository](https://github.com/hiibolt/niu-metis-documentation/tree/main/projects).

### Explored Use Cases
There are six use cases covered here, with increasing levels of control over Metis:
* **Chapter 2.1 - Running a C++ project**
    - No additional configuration
    - PBS only
* **Chapter 2.2 - Running a CUDA project**
    - Loading CUDA via the `module` command
    - PBS only
* **Chapter 4.1 - Running a language not installed on Metis, such as Python 3.11**
    - Downloading a pre-built Docker Image with `python` version 3.11 installed
    - PBS with Docker via Podman
* **Chapter 4.2 - Running packages not installed on Metis with GPU passthrough**
    - Downloading a pre-built Docker Image with the requirements for CUDA
    - Passing through GPUs to Docker
    - PBS with Docker and NVIDIA Container Toolkit via Podman
* **Chapter 4.3 - Running virtually any project using custom Docker Images**
    - Writing, building, and publishing your own Docker Image
    - Passing through GPUs to Docker
    - PBS with Docker and NVIDIA Container Toolkit via Podman
* **Chapter 5.1 - SSH Automation**
    - Demonstrates programmatic submission of PBS jobs via SSH for the purpose of fitting Metis into existing systems.

## Where Should I Read to?
### Cases Where Docker May Not Be Needed
If your application is either of the following, you shouldn't use Docker.
- Native C, Go, or Python applications *with pre-installed or no dependencies*
- OpenMPI-based applications

If it's one of those two, chapters **2.1** and **2.2** will be of great use!

The following chapters may not be as useful, as they touch primarily on Docker.
### Cases Where Docker Is Needed
If your application is any of the following, it's highly recommended to use Docker: 
- Applications which require a different operating system
- Applications that are not pre-installed and easier to setup using docker than natively (consult with [crcdhelpdesk@niu.edu](mailto:crcdhelpdesk@niu.edu))

If you only need CPU-based computation, chapters **2-2.2 and 4.1** will teach you everything you need.

If you need GPU passthrough or have a very complicated project, it is recommended to read this book in its entirety!