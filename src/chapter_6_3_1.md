# 6.3.1. Creating Modulefiles
<small>*Associated CRCD Documentation: [Modules](https://crcd.niu.edu/crcd/current-users/crnt-users-software.shtml)*</small>

*You can find the code mentioned in this chapter [in this book's repository](https://github.com/hiibolt/niu-metis-documentation/tree/main/projects/modulefile/hello_metis)!*

This guide will give you a minimally reproducible example of how to create your own modules which can be loaded via the `module` command.

## Creating the `modulefile`

First, we will start by creating a directory to house our personal modules, and another for our "Hello, Metis!" project:
```bash
$ mkdir ~/modules
$ mkdir ~/modules/hello_metis
$ mkdir ~/modules/hello_metis/bin
$ cd ~/modules/hello_metis
```

Next, let's create our binary from a C++ source:
```bash
$ touch main.cpp
```

In the `main.cpp` file, write the following contents:
```c++
#include <iostream>

int main () {
    std::cout << "Hello, Metis!" << std::endl;
}
```

Now, let's compile it, and place it in our `bin` folder:
```bash
$ g++ -o bin/hello_metis main.cpp
```

The final step is creating our `modulefile`, which we will name `hello_metis-0.0.1`:
```bash
$ touch hello_metis-0.0.1
```

Creating a modulefile is a surprisingly difficult task, but the minimal reproducible example is the following contents:
```bash
#%Module

# Add the bin folder from the `~/modules/hello_metis/bin` directory to the PATH
prepend-path PATH $env(HOME)/modules/hello_metis/bin
```

## Loading a custom source

Before we can use it, we need to add our custom `~/modules` folder as a source that `module` can then understand. It's worth noting that you will need to redo this each time you wish to load your custom modules! Using our `modules` directory:
```
$ module use ~/modules
```

Then, we can load our custom `modulefile`, and test it out:
```
$ module load hello_metis/hello_metis-0.0.1
$ hello_metis
Hello, Metis!
```

Congratulations! This is a fully functional `module` setup, but you would ideally want to improve upon this greatly.

To learn more about writing `modulefiles`, see the [official Modules documentation](https://modules.readthedocs.io/en/v5.4.0/modulefile.html#description).