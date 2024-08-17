# 4.1. SSH Automation with Metis
*You can find the code mentioned in this chapter [in this book's repository](https://github.com/hiibolt/niu-metis-documentation/tree/main/projects/rust)!*

While Metis is an incredibly powerful tool, it does not provide native tooling to allow for automatic job submission.

One solution is to write our own software which submits the job on our behalf, using SSH-related libraries to open a connection and submit commands!


## Goals
* Learn how to automate an SSH session and commands
* Learn how to add your system as a known host
* Understand the importance of hardening your code

## The Problem(s)
First, let's talk about what Metis can and can't do.

There are a few problems with automation on Metis that make it more difficult than a standard server:
* **You cannot host a webserver on Metis**
* **Ports cannot be forwarded**

This means that one cannot simply host a webserver, which could otherwise recieve requests to start jobs automatically.

So, what can we do?

## The Solution
When asking why you can't automate something, one of the first questions is to ask *"Well, how am I able to do it manually?"*.

In this case, we are using SSH to connect, and we are then running `qsub` to submit our jobs.

Well, can that be done programmatically? 

Yes, but it's a little more complicated than doing it by hand.

## Implementation
For the sake of this guide, I will be using the [Rust programming language](https://www.rust-lang.org/). This is a programming language that best illustrates potential failure points in a program, forcing you to cover error cases in advance.

SSH has many potential points of failure, so using it can help you to think ahead to cover your bases!

However, you don't need to use Rust, you can just as easily write your connection code in Python, C, or any language that suits your need - as long as you write code that can handle and communicate failure well.

For instance, here is example Rust code to submit a `qsub` job (if you would like to follow along, please see the repository [here](https://github.com/hiibolt/niu-metis-documentation/tree/main/projects/rust)!):
```rust
use openssh::{Session, KnownHosts};

async fn submit_pbs_job (
    username: &str,
    path: &str,
    arguments: Vec<(&str, &str)>
) -> Result<String, String> {
    // Open a multiplexed SSH session
    let session = Session::connect_mux(&format!("{username}@metis.niu.edu"), KnownHosts::Strict).await
        .map_err(|err| format!("Couldn't connect to METIS! Are your credentials correct? Raw error:\n{err}"))?;

    // Build and run the `qsub`` command
    let mut submit_job_command_output = session
        .command("qsub");

    // Build the arguments string
    let stringified_arguments = arguments
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<String>>()
        .join(",");

    // Append the arguments string to the command, if there are any arguments
    let submit_job_command_output = if stringified_arguments.len() > 0 {
        submit_job_command_output
            .arg("-v")
            .arg(stringified_arguments)
    } else {
        &mut submit_job_command_output
    };

    // Append the job script path to the command
    let submit_job_command_output = submit_job_command_output
        .arg(path)
        .output().await
        .map_err(|err| format!("Failed to run qsub command! Raw error:\n{err}"))?;

    // Check if the command was successful
    if !submit_job_command_output.status.success() {
        let err = String::from_utf8(submit_job_command_output.stderr)
            .map_err(|err| format!("Failed to decode the error message! Raw error:\n{err}"))?;

        return Err(format!("When running the qsub command, the following error occurred:\n{err}"));
    } 

    // Otherwise, return the output (as a string)
    let successful_output = String::from_utf8(submit_job_command_output.stdout)
        .map_err(|err| format!("Failed to decode the output message! Raw error:\n{err}"))?;

    Ok(successful_output)
}

#[tokio::main]
async fn main() {
    // Submit a job to the METIS cluster
    let job_id = submit_pbs_job("z1994244", "/home/z1994244/projects/cpp/hello_world/run.pbs", vec![
        ("ARGUMENT_1", "VALUE_1"),
        ("ARGUMENT_2", "VALUE_2"),
        ("ARGUMENT_3", "VALUE_3"),
    ]).await;

    // Check if the job was submitted successfully
    match job_id {
        Ok(job_id) => println!("Job submitted successfully! Job ID: {job_id}"),
        Err(err) => eprintln!("Failed to submit the job! Error message:\n{err}"),
    }
}
```

Our first step is to use an SSH library - in this case, the crate `openssh` - to open a [multiplexed](https://en.wikibooks.org/wiki/OpenSSH/Cookbook/Multiplexing) SSH connection. 

Many other libraries exist for other languages, such as `ssh-python` for Python and `ssh` for Go.

However, it's worth noting just how many potential points of failure there are:
* The SSH can fail to open because you weren't a known host
* The command can fail to send over SSH
* The `qsub` command can fail (on Metis' end), and return an error
* The `stderr` from reading the failure reason from Metis can provide invalid UTF-8 (unlikely, but possible!)
* The output from `stdout` of the `qsub` command can provide invalid UTF-8 (unlikely, but possible!)

The first failure will likely happen, unless you've aleady made Metis a known host on the system you will be automating SSH from.

So, how do we add Metis as a known host? We need to create an SSH key, and copy it over to Metis. This allows Metis to skip password-based authentication thanks to knowing it's us!

You can hit enter through all of the prompts in the `ssh-keygen` command, but run the following **on your host machine, not Metis**:
```
$ ssh-keygen
$ ssh-copy-id <your_account_username>@metis.niu.edu
```
 
Now that Metis is a known host, we can test our program.

If you are following along with this tutorial in Rust, you can find the codebase [here](https://github.com/hiibolt/niu-metis-documentation/tree/main/projects/rust), as you'll need to have the `openssh` and `tokio` crates installed and configured.

Testing our program:
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/igait-ssh-testing`
Job submitted successfully! Job ID: 18734.cm
```

Congratulations! It worked, and you've just submitted a PBS job automatically!

## Important Notes
Many `openssh` implementations, including in Rust, only run commands from the home directory. In some implementations, you can change this, but in many, you cannot. This is why, throughout our projects, we've been providing absolute paths. Otherwise, the `$PBS_O_WORKDIR` for our SSH automation would resolve to `~/.`, which would cause unexpected failures.

By writing our paths in absolute, we guarantee proper execution.

Now, where is our output? Well, as previously mentioned, often, commands are run from the `~/.` (home) directory. Sure enough, after manually logging into Metis:
```
$ ls
bin  examples  hello_world.o18734  projects  rundir
```

While not shown here, it is possible to automatically read the contents of this output folder, using a `cat` command or the likes after the expected run time is over.

It cannot be understated how important it is that you are extremely careful whenever automating your workflow!

You must purify your inputs, and ensure it is physically impossible for an attacker to exploit your backend in any way possible. To not do so would endanger the work of fellow NIU researchers, students and staff.

However, as mentioned in the preface to this chapter, it's an incredibly effective method that can be further evolved into even more effecient and better integrated systems!