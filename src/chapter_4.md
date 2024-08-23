# 4. Advanced Metis Usage Techniques
One of the most powerful tricks on Metis is SSH automation.

This allows a Metis user to automate what would otherwise be:
* **1** - Logging into Metis over SSH
* **2** - Running job submission commands
* **3** - Retrieving a job ID

By doing this, we can intergrate Metis into the workflow of any existing web server!

This technique also opens the door to other techniques, three of which will be briefly mentioned in **Chapter 4.2 - Conceptual Techniques**. As the title states, because of the varied and complex nature of implementation they will only be described conceptually.

## Overview of the Chapters

### Chapter 4.1: SSH Automation with Metis
* **Goals**: Learn how to automate SSH commands with Metis..
* **Problem**: Metis does not allow webservers, making automation difficult.
* **Solution**: Use an SSH library to open a multiplexed connection for execution.
* **Outcome**: You will be able to run any command on Metis programmatically.

### Chapter 4.2: Using GPU Acceleration with Docker
* **Goals**: Learn how to enable GPU passthrough in Docker containers on Metis.
* **Problem**: GPUs are not accessible in Docker by default, and additional steps are required to set up NVIDIA drivers and CUDA.
* **Solution**: Configure Podman with specific flags and use NVIDIA's CUDA-enabled Docker images.
* **Outcome**: You will be able to leverage GPU acceleration for your Dockerized applications, significantly boosting performance.

### Chapter 3.3: Creating Your Own Docker Base Images
* **Goals**: Gain the skills to create custom Docker images tailored to your project’s needs.
* **Problem**: Pre-made Docker images may not always meet the specific requirements of your project.
* **Solution**: Learn the basics of writing Dockerfiles, building custom images, and publishing them to public repositories.
* **Outcome**: You will be able to create, customize, and share Docker images, enabling a flexible and reproducible environment for your work.
