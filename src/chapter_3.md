# 3. Intermediate Metis Usage with Metis

In this chapter, we will dive deep into Docker and PBS techniques, expanding on the foundational knowledge covered in Chapters 2.1 and 2.2.

We'll explore how to combine these techniques to create a robust workflow for your projects on Metis, including handling custom Docker images, leveraging GPU acceleration, and managing complex dependencies.

## Podman + Docker vs Singularity/Singularity + Docker
Metis has both Podman *and* Singularity installed. Both are software designed to allow non-root users to run containers on systems like Metis, where this is the case.

Podman is designed to effectively be a drop-in replacement for Docker on non-root systems, while Singularity is a tool specifically designed for HPC environments with performance, reproducibility, and security in mind.

Although this documentation does not cover Singularity, it is a very powerful tool that should be considered if you are looking to squeeze the maximum performance out of Metis.

**Pros**:
* Native OpenMPI Support - Docker struggles with OpenMPI
* Improved Security - Other users cannot easily see your work
* HPC-first Design - Singularity is *very* well optimized for performance

**Cons**:
* Significantly Smaller Ecosystem + Image Repository - Singularity is HPC-first, meaning there are far less users, and accordingly far fewer help resources available
* Less Flexible - Singularity's focus on reproducibility can make creating images more tedious, and can even eliminate the possibility of creating certain applications
* Difficult to Test Locally - The process of getting started on a local system is more complex and less beginner-friendly

**Comparison Chart**:
| Feature                    	| Podman + Docker                 	| Singularity                   	|
|--------------------------------|------------------------------------|----------------------------------|
| **Use Case**                	| General development, CI/CD, image availability | HPC environments, scientific computing |
| **Image Repository**        	| Docker Hub (large and varied)  	| Focus on reproducibility, often specific images |
| **Security**                	| Rootless operation (Podman)     	| High security in multi-user environments |
| **Integration with HPC**    	| Limited                          	| Optimized for HPC environments   |
| **Ease of Use in HPC**      	| Requires additional configuration	| Seamless integration with HPC systems |

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
