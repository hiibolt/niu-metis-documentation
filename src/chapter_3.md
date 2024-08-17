# 3. Intermediate Metis Usage with Metis

In this chapter, we will dive deeper into advanced Docker techniques, expanding on the foundational knowledge covered in Chapters 2.1 and 2.2.

We'll explore how to combine these techniques to create a robust workflow for your projects on Metis, including handling custom Docker images, leveraging GPU acceleration, and managing complex dependencies.

## Overview of the Chapters

### Chapter 3.1: Using Pre-Made Docker Images
* **Goals**: Understand the limitations of Metis modulefiles and learn how to circumvent them using Docker.
* **Problem**: Some software, like Python 3.11, isn't available on Metis, and creating modulefiles can be time-consuming.
* **Solution**: Use Docker (via Podman) to run applications with custom dependencies.
* **Outcome**: You will be able to run any version of software, regardless of the limitations of the Metis environment.

### Chapter 3.2: Using GPU Acceleration with Docker
* **Goals**: Learn how to enable GPU passthrough in Docker containers on Metis.
* **Problem**: GPUs are not accessible in Docker by default, and additional steps are required to set up NVIDIA drivers and CUDA.
* **Solution**: Configure Podman with specific flags and use NVIDIA's CUDA-enabled Docker images.
* **Outcome**: You will be able to leverage GPU acceleration for your Dockerized applications, significantly boosting performance.

### Chapter 3.3: Creating Your Own Docker Base Images
* **Goals**: Gain the skills to create custom Docker images tailored to your project’s needs.
* **Problem**: Pre-made Docker images may not always meet the specific requirements of your project.
* **Solution**: Learn the basics of writing Dockerfiles, building custom images, and publishing them to public repositories.
* **Outcome**: You will be able to create, customize, and share Docker images, enabling a flexible and reproducible environment for your work.
