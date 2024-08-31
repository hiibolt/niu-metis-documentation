# 1.1. Connecting to Metis
<small>[*Associated CRCD Documentation*](https://crcd.niu.edu/crcd/current-users/getting-started/login-to-metis.shtml)</small>

The Metis cluster is easily accessible via the [SSH](https://en.wikipedia.org/wiki/Secure_Shell) protocol.

Each operating system has various possible SSH clients - we will be using the OpenSSH client, as it is pre-installed on most operating systems and very straightforward.

## Windows 10 and 11
As of 2024, both Windows 10 and Windows 11 have the OpenSSH client pre-installed. If you don't have it installed, update your operating system using the Windows Updater.

To connect, open either **Windows PowerShell** or **Command Prompt**, and run the following:
```ps
PS C:\...\> ssh you@metis.niu.edu
```

When prompted, enter your temporary password.

On your first login, you will be prompted to create a new password. Ensure that it's something memorable, but very secure!

Then, close your session:
```bash
[you@metis ~]$ exit
```

And re-login to test your new password:
```ps
PS C:\...\> ssh you@metis.niu.edu
```

## Linux and MacOS
Most major distributions and the latest versions of MacOS have the OpenSSH client installed.

Run the following:
```bash
$ ssh you@metis.niu.edu
```

When prompted, enter your temporary password.

On your first login, you will be prompted to create a new password. Ensure that it's something memorable, but secure!

Then, close your session:
```bash
[you@metis ~]$ exit
```

And re-login to test your new password:
```bash
$ ssh you@metis.niu.edu
```