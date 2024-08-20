# Podman Quick Reference

## Cleaning Your User Directory
It's very important that you routinely run `/bin/podman system prune`. The reason for this is because Podman *can't run cleanup commands* if you don't have any remaining disk quota!

Because Metis users (without special request) have a maximum home (`~/*`) directory size of ~25GB, it's critical to stay under this limit while developing with Podman.

*If you run into this issue, either contact CRCD to have your quota temporarily increased, or see **Force Cleaning Podman**!*

### Force Cleaning Podman
If you're not careful and don't routinely clean Podman, you might reach a stalemate where you can't do anything on Metis because you have no disk quota, but you also can't use Podman's cleaning utilities!

Example:
```
$ podman inspect
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