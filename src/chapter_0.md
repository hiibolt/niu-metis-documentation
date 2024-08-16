# 1. Introduction
Welcome! This book serves as an all-in-one crash course in utilizing Metis, as well as some advanced techniques.

While this book does assume a basic understanding of Linux and C++, it is not required.

If you have not logged into Metis before, or if it's been some time since you've used Linux, NIU CRCD has [comprehensive documentation](https://www.niu.edu/crcd/current-users/getting-started/login-to-metis.shtml) on how to open a SSH connection, and a quick Linux refresher.

If you would also like to refresh yourself on C++, w3schools has a wonderful [quick reference](https://www.w3schools.com/cpp/default.asp) available.

Although this guide is about how to employ Docker on Metis, it does not assume you have previous experience with it, and includes a crash course on basic usage. The skills in that section provide tools to build even some of the most advanced applications, and also includes resources for those applications which require more.

## Primary Purpose
The goal of this book is to allow researchers at NIU to hit the ground running with their research. Our goal is for you to focus less on getting Metis to work for you - and more on completing your work as a whole.

It's possible to create applications that will work regardless of what Metis has available to you for installation. Furthermore, you no longer have to develop directly on Metis - you can develop and build locally on hardware you're used to. This will allow you to focus on writing your application - without having to worry whether it can run on Metis.

We will do so by employing [Docker](https://www.docker.com/), an extremely powerful containerization and encapsulation tool that allows developers to define virtual machines with a level of granularity rarely found in modern computing. Docker allows you to select an operation system as a base, install packages and libraries, and define run behaviour.

All of this is defined in a singular, simple, and human-readable file that can be build to be reproduced on any system - including Metis.


### Explored Use Cases
There are five use cases covered here, with increasing levels of control over Metis:
* **Running a C++ project**
    - No additional configuration
    - PBS only
* **Running a CUDA project**
    - Loading CUDA via the `module` command
    - PBS only
* **Running a language not installed on Metis, such as Rust**
    - Downloading a pre-built Docker Image with `cargo` installed
    - PBS with Docker via Podman
* **Running packages not installed on Metis with GPU passthrough**
    - Downloading a pre-built Docker Image
    - Passing through GPUs to Docker
    - PBS with Docker and NVIDIA Container Toolkit via Podman
* **Running virtually any project using custom Docker Images**
    - Writing, building, and publishing your own Docker Image
    - Passing through GPUs to Docker
    - PBS with Docker and NVIDIA Container Toolkit via Podman

### Advanced Techniques
This guide will also explore one additional advanced technique:
* **SSH Automation**
    - Demonstrates programmatic submission of PBS jobs via SSH for the purpose of fitting Metis into existing systems.