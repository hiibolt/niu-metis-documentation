# 5.3. Modules
<small>*Associated CRCD Documentation: [Modules](https://crcd.niu.edu/crcd/current-users/crnt-users-software.shtml)*</small>

A module is a set of one or more packages used to extend the functionality of Metis with applications, libraries, or dependencies.

On Metis, `module` is effectively the package management system.

## Primary Commands
### `$ module av`
Lists all available packages, sorted by each available source.

Common Options:
* `<keyword>` 

    This will filter output by packages of which the keyword is in the name of.

    Example:
    ```bash
    $ module av python
    ----------------------- /etc/modulefiles ------------------------
    python/python-3.9.10  python/python-3.12.4
    ```

### `$ module load <module_name>`
Loads a package by name from the available sources.

Example:
```bash
$ module load gcc/gcc-5.5.0
```

*To list or add to available sources, see `module use`.*

### `$ module unload <module_name>`
Unloads a package by name from the currently loaded modules.

Example:
```bash
$ module load gcc/gcc-5.5.0
$ module unload gcc/gcc-5.5.0
```

### `$ module purge`
Unloads all currently loaded packages

Example:
```bash
$ module purge
```

## Other Commands
### `$ module use <path_to_source>`
Ephemerally adds a source to the `module` commands. This means you can add your own modules to the `<path_to_source>`, and be able to load and unload them.

Omitting the path will instead print a list of currently linked sources.

Example (see **Chapter 5.4.1** for an in-depth example):
```bash
$ module use ~/modules
$ module load my_package/my_package-0.0.1
```

### `$ module list`
Lists the loaded modulefiles.

Example:
```bash
$ module list
Currently Loaded Modulefiles:
 1) hello_metis/hello_metis-0.0.1
```

### `$ module switch <module_1> <module_2>`
Unloads `<module_1>` and instead loads `<module_2>`.

Example:
```bash
$ module load gcc/gcc-9.5.0
$ module switch gcc/gcc-9.5.0 gcc/gcc-4.9.3
```

### `$ module help <module_name>`
Prints the help information on a module, if it exists.

Example:
```
$ module help gcc/gcc-9.5.0
-----------------------------------------------------------------
Module Specific Help for /etc/modulefiles/gcc/gcc-9.5.0:

This module loads GCC gcc/gcc-9.5.0
-----------------------------------------------------------------
```