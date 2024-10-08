# 6.1. Bash

This section is a summary of the most common [Bash](https://www.gnu.org/software/bash/) commands to serve as a general introduction or refresher to help newer Linux users, or those who have not used it in some time.

Interested in learning even more about Bash? The [GNU Bash Reference](https://www.gnu.org/software/bash/manual/bash.html) is an amazing resource!

## Common Commands
### `$ ls`

Lists the contents in the current directory.

Example:
```
[you@metis.niu.edu ~]$ ls
bin examples 
```

Common Arguments:
* `-a`

    Prints everything, including hidden files.

    Example:
    ```
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

<small>*[Command Manual](https://www.man7.org/linux/man-pages/man1/ls.1.html)*</small>


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

<small>*[Command Manual](https://man7.org/linux/man-pages/man1/cd.1p.html)*</small>

### `$ touch <file_name | file_name>`

Creates a new file with empty contents.

Example:
```bash
[you@metis ~]$ touch hello.txt
[you@metis ~]$ ls
bin projects hello.txt
```

<small>*[Command Manual](https://man7.org/linux/man-pages/man1/touch.1.html)*</small>

### `$ nano <file_name | file_path/file_name>`

A simplistic terminal file editor, useful for quick edits.

Shouldn't be used for large files; instead, you should use `emacs`, `vim`, or ideally, an editor on *your* machine with remote SSH capability. See **Chapter 1.1** for more information on setting up Visual Studio Code, a popular option.

Example:
```bash
[you@metis ~]$ touch hello.txt
[you@metis ~]$ nano hello.txt
```

<small>*[Command Manual](https://www.nano-editor.org/dist/v2.1/nano.html)*</small>

### `$ mkdir <dir_name | dir_path/dir_name>`

Creates a new and empty directory.

Example:
```
[you@metis.niu.edu ~]$ mkdir hello
[you@metis.niu.edu ~]$ ls
bin examples hello
```

<small>*[Command Manual](https://www.man7.org/linux/man-pages/man1/mkdir.1.html)*</small>

### `$ export <var>=<string | expression>`

Sets an environment variable. Unless somehow preserved, these will be cleared when you close the session!

Example:
```
[you@metis.niu.edu ~]$ export FOO="bar"
```

<small>*[Command Manual](https://www.man7.org/linux/man-pages/man1/export.1p.html)*</small>

### `$ echo <string | expression>`

Outputs the specified string or expression to stdout (the terminal).

You can output environment variables by prefacing a variable name with `$`.

Example:
```
[you@metis.niu.edu ~]$ echo "Hello, Metis!"
Hello, Metis!
[you@metis.niu.edu ~]$ export FOO="Hello, Metis!"
[you@metis.niu.edu ~]$ echo "$FOO"
Hello, Metis!
```

<small>*[Command Manual](https://www.man7.org/linux/man-pages/man1/echo.1.html)*</small>

## Help Commands

Should you feel confused on the usage of any command, you can print additional helpful information on many commands!

The 5 common ways to print help on a command, in order of the density of information output:
* `$ info <command>`
* `$ man <command>`
* `$ <command> --help`
* `$ <command> -h`
* `$ <command> -?`

Generally, first try `$ <command> --help`, and if you're still confused, try `$ man <command>`.