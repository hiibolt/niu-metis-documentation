# 3. User Environment Customization/Virtualization on Metis

## Podman + Docker vs Singularity/Apptainer + Docker vs Conda vs Modulefiles

Metis provides several technologies to encapsulate environments and manage software dependencies. Each of these tools—Podman, Docker, Singularity/Apptainer, Conda, and Modulefiles—has different strengths and weaknesses, depending on your use case.

Any technology marked with a "⭐" is a strong candidate in its given feature.

## Comparison Chart
| Feature                      | Podman + Docker                 | Singularity/Apptainer           | Conda                             | Modulefiles                         |
|------------------------------|----------------------------------|---------------------------------|-----------------------------------|-------------------------------------|
| **Use Case**                  | ⭐ General development, CI/CD, image availability | ⭐ HPC environments, scientific computing | Managing isolated software environments | Dynamic loading of software on HPC clusters |
| **Image/Package Repository**  | ⭐ Docker Hub (extremely large and varied)    | Smaller ecosystem, focus on reproducibility | Anaconda repository <br> *(⭐ only if using Python or R)*  | Pre-installed software for the HPC cluster |
| **Security**                  | Rootless operation (Podman)      | ⭐ High security in multi-user environments | No OS-level isolation              | ⭐ Tied to user permissions on HPC     |
| **Integration with HPC**      | Limited, requires config         | Optimized for HPC environments  | Limited, not designed for HPC      | Native integration with HPC systems |
| **Ease of Use in HPC**        | Very easy, but requires some minimal configuration | Tedious but straightforward to work with                | Simple for general use (but not HPC-optimized) | ⭐ Extremely easy to use, but requires cluster-specific knowledge |