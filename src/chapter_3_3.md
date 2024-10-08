# 3.3. Pros/Cons: Modulefiles
<small>*To learn more about Modulefiles or to find links to CRCD's documentation, see **Chapter 6.3**!*</small>

**Environment Modules** (or **Modulefiles**) are a system for dynamically modifying user environments via modulefiles. They are heavily used in HPC environments to load software environments on-demand without needing root privileges.

**Pros**:
- **Lightweight**: No overhead from containerization.
- **HPC Optimized**: Designed specifically for HPC environments, often with pre-built software optimized for the specific cluster. Easily the most optimized technology.
- **Flexible**: Allows for loading different software versions, easy to use and understand in HPC.

**Cons**:
- **Not Portable**: Modules are often tightly coupled to the cluster’s software stack, making them difficult to reproduce elsewhere.
- **Manual Management**: Requires explicit loading/unloading, making it less automated compared to container technologies.
- **Complex and Administrator Dependent**: You may need administrator intervention to add or modify system modules.
