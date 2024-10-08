# 3.1 Pros/Cons: Podman + Docker and Singularity/Apptainer + Docker
<small>*To learn more about Podman, see the [official Podman Documentation](https://podman.io/docs), or a great [beginner's tutorial](https://devopscube.com/podman-tutorial-beginners/)!*</small>

<small>*To learn more about Apptainer, see the [official Apptainer Documentation](https://apptainer.org/), or a comprehensive [tutorial](https://hsf-training.github.io/hsf-training-singularity-webpage/)!*</small>

Metis has both Podman *and* Singularity installed. Both are software designed to allow non-root users to run containers on systems like Metis, where this is the case.

Podman is designed to effectively be a drop-in replacement for Docker on non-root systems, while Singularity is a tool specifically designed for HPC environments with performance, reproducibility, and security in mind.

Although this documentation does not cover Singularity, it is a very powerful tool that should be considered if you are looking to squeeze the maximum performance out of Metis.

### Podman + Docker
Podman and Docker are containerization platforms that allow users to run and manage containers in isolated environments. 

**Pros**:
- **Large Ecosystem**: Docker has an extensive library of images on Docker Hub.
- **Rootless Operation (Podman)**: Podman is rootless by default, improving security without compromising functionality.
- **Versatile**: Ideal for general development, CI/CD pipelines, and application isolation.

**Cons**:
- **Limited HPC Integration**: Requires extra configuration to integrate with HPC systems, especially regarding MPI.
- **Overhead**: Containerization can add overhead compared to native execution, especially for complex MPI-based workflows.

### Singularity/Apptainer
Singularity, now rebranded as **Apptainer**, is a containerization technology specifically designed for HPC environments. Apptainer allows users to encapsulate applications and their dependencies in containers that are highly portable and optimized for performance in multi-user systems.

**Pros**:
- **Native OpenMPI Support**: Singularity/Apptainer handles MPI seamlessly, outperforming Docker in HPC scenarios.
- **Reproducibility**: Ensures consistent performance and results, vital in scientific computing.
- **Security**: Designed for multi-user environments, ensuring other users can't interfere with your containers.

**Cons**:
- **Smaller Ecosystem**: Fewer available container images and less community support compared to Docker.
- **Less Flexibility**: Its focus on reproducibility can make certain application deployments more challenging.
- **Complex Local Setup**: Initial setup on local machines can be tricky, especially compared to Docker/Podman.
