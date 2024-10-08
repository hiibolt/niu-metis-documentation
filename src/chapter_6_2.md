# 6.2. Podman and Docker Quick Reference

This section summarizes the most important commands Podman and Docker offer.

It's worth noting that there are many, many more commands both offer, so check out their respective references if you're looking for more!
* [Docker's Documentation](https://docs.docker.com/)
* [Podman's Documentation](https://docs.podman.io/)

*Podman is a proxy layer on Docker, which means all commands (listed in this reference) are the same regardless of whether you use them on your machine with `docker` or on Metis with `/bin/podman`.*

**Notes**:
* Staying Tidy

    It's very important that you routinely run `/bin/podman system prune`. The reason for this is because Podman *can't run cleanup commands* if you don't have any remaining disk quota!

    Because Metis users (without special request) have a maximum home (`~/*`) directory size of ~25GB, it's critical to stay under this limit while developing with Podman.

    *If you run into this issue, either contact CRCD to have your quota temporarily increased, or see **Addendum - Force Cleaning Podman**!*
* Errors Which Can be Ignored

    * ```ERRO[0000] cannot find UID/GID for user z1994244: no subuid ranges found for user "z1994244" in /etc/subuid - check rootless mode in man pages.```

        This error sometimes occurs the first time you run a command.

        If it does, simply wait a few seconds and run it again.

    * ```WARN[0000] Network file system detected as backing store.  Enforcing overlay option `force_mask="700"`.  Add it to storage.conf to silence this warning```

        At the time of writing this, you will see this quite often. It can be safely ignored.

        It's an artifact of using Podman, and will hopefully be fixed in the future.

## Primary Commands
### `$ /bin/podman run image:tag`
Starts a container, pulling the image if needed.

If you don't specify a name, it will output the newly allocated container ID.

```bash
[you@metis.niu ~]$ /bin/podman run --name python_container python:3.12.5-bookworm
...
b647edca4b32eb02d15dc8cb70dc2a3da8edcf9e767c1f3ff2d7a58133ce407c
```

Common Arguments:
* `--name <container_name>`

    Gives the container a name, which can be used conveniently in place of the container's ID.
* `-t`

    Allocates a psuedo-TTY (helps support console-based applications)
* `-d`

    Runs the container in detached mode.

    Without this option, standard input, output, and error are linked to yours.
* `-v <host_path>:<container_path>`

    Mounts a path from your host machine to the container as a Docker Volume.

    Very useful for easily importing your project directory.
* `-w <container_path>`

    Changes the working directory inside the container to the specified path.

<small>*[Command Manual](https://docs.podman.io/en/latest/markdown/podman-run.1.html)*</small>

### `$ /bin/podman exec <container_id | container_name> <command>`
Executes a command in a container.

Example:
```bash
$ /bin/podman exec python_container python3 main.py
```

### `$ /bin/podman kill <container_id | container_name>`
Attempts to stop a container by sending the `SIGKILL` signal.

Example:
```bash
$ /bin/podman kill python_container
```

<small>*[Command Manual](https://docs.podman.io/en/latest/markdown/podman-kill.1.html)*</small>

### `$ /bin/podman rm <container_id | container_name>`
Removes a container, very useful for reclaiming the name of a container.

For instance, even if you were to kill `python_container` with `/bin/podman kill`, you still would not be able to create a new container with the name `python_container`. You must also remove the original.

Example:
```bash
$ /bin/podman rm python_container
```

<small>*[Command Manual](https://docs.podman.io/en/latest/markdown/podman-rm.1.html)*</small>

## Addendum
### Force Cleaning Podman
If you're not careful and don't routinely clean Podman, you might reach a stalemate where you can't do anything on Metis because you have no disk quota, but you also can't use Podman's cleaning utilities!

Example:
```
$ /bin/podman inspect
...

Error: close /home/<your_metis_username>/.local/share/containers/storage/overlay/.has_mount_program: disk quota exceeded
```

First, confirm it's Podman using most of your storage:
```bash
du -sh ~/.local/share/containers/storage/overlay
26GB
```

Then, we will manually delete the `overlay` directory. It's currently unclear what side effects manually performing this action does, so it may be better to have your quota increased instead. The commands will be listed below, nonetheless:
```bash
$ rm -rf ~/.local/share/containers/storage/overlay
$ mkdir ~/.local/share/containers/storage/overlay
```

Then, you'll need to "reset" Podman:
```bash
$ /bin/podman system reset
```