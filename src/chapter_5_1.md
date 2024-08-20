# 5.1. Bash Commands

This section is a summary of the [Bash Reference](https://www.gnu.org/software/bash/manual/bash.html) to serve as a general introduction or refresher on some common [Bash](https://www.gnu.org/software/bash/) commands to help newer Linux users, or those who have not used it in some time.

## Common Commands
### `$ ls`

Lists the contents in the current directory.

Example:
```
$ ls
bin examples 
```

Common Arguments:
* `-a`

    Prints everything, including hidden files.

    Example:
    ```bash
    [you@metis ~]$ ls -a
    .              .dbus           .kde
    ..             .dotnet         .kshrc   
    .bash_history  .emacs          .local           
    .bash_logout   .esd_auth       .mozilla       
    .bash_profile  examples        .ssh       
    .bashrc        .gitconfig      .wget-hsts      
    bin            .nv             .Xauthority
    .cache         .python_history .xemacs
    .config        .jupyter        .zshrc
    ```


### `$ cd <path>`

Changes the current directory to the specified path.

There are a few types of paths in a Unix-based filesystem, mainly being:
* Absolute Path

    Absolute paths always lead to the same location, no matter the context they are interpreted from.

    They typically start with `/`, which is the root (base level) of the filesystem, but they can also start with `~`, which is your home directory.

    For example, your home directory (akin to a desktop in a graphical OS) is at `/home/you` or `~`.
* Relative Path

    Relative paths are dependent on where they are run from, and are specified by *not* starting with a `/`.

    For example, if you are in your home directory, the `bin` directory can be referenced by `./bin`. 

    The `.` signifies "current directory", but you can also use "..", which would represent "up one directory".

Here is an example of changing to your `bin` directory based on an absolute path:
```bash
[you@metis ~]$ cd /home/you/bin
[you@metis bin]$
```

*(`cd ~/bin` would be equivalent!)*

Changing directory to your `bin` directory relative to your current directory (that being `~`):
```bash
[you@metis ~]$ cd bin
[you@metis bin]$
```

*(`cd ./bin` would be equivalent!)*

Going up a directory, then into the examples directory:
```bash
[you@metis bin]$ cd ../examples
[you@metis examples]$
```

### `$ touch <file_name | file_name>`

Creates a new file with empty contents.

Example:
```bash
[you@metis ~]$ touch hello.txt
[you@metis ~]$ ls
bin projects hello.txt
```

### `$ nano <file_name | file_path/file_name>`

A simplistic terminal file editor, useful for quick edits.

Shouldn't be used for large files; instead, you should use `emacs`, `vim`, or ideally, an editor on *your* machine with remote SSH capability. See **Chapter 1.1** for more information on setting up Visual Studio Code, a popular option.

Example:
```bash
[you@metis ~]$ touch hello.txt
[you@metis ~]$ nano hello.txt
```

## Help Commands

Should you feel confused on the usage of any command, you can print additional helpful information on many commands!

The 5 common ways to print help on a command, in order of the density of information output:
* `$ info <command>`
* `$ man <command>`
* `$ <command> --help`
* `$ <command> -h`
* `$ <command> -?`

Generally, first try `$ <command> --help`, and if you're still confused, try `$ man <command>`.