# 5.4. PBS Professional
<small>*Associated CRCD Documentation: [Modules](https://crcd.niu.edu/crcd/current-users/getting-started/run-interactive-jobs.shtml)*</small>

A summarization of the [PBS Professional user's guide by Altair](https://2021.help.altair.com/2021.1.2/PBS%20Professional/PBSUserGuide2021.1.2.pdf) for the [PBS Professional](https://altair.com/pbs-professional/) system.

## Primary Commands
### `$ qsub <script_path>`
Submits an executable PBS script to a batch server, outputting the ID of the newly created job.

Example:
```bash
$ qsub ./run.pbs
20000.cm
```

Common Options:
* `-v variable_list`

    Adds additional environment variables to the context of the PBS script.

    Multiple variables can be seperated by a comma, and an environment variable can be extracted from the context where `qsub` is invoked by *not* providing a value with an `=` sign.

    Example:
    ```bash
    $ qsub -v foo=bar,lorem ./run.pbs
    20000.cm
    ```
* `-V`

    Exports all environment variables in the context where `qsub` is invoked to the context of the PBS script.

    Example:
    ```bash
    $ qsub -V ./run.pbs
    20000.cm
    ```
* `-I`

    Makes your job psuedo-interactive, connecting standard input, output, and error streams to the context where `qsub` is executed from.

    This can be useful for creating programs where input is required and would otherwise time out.

    Example:
    ```bash
    $ qsub -I ./mult_by_two.pbs
    Enter the number you want to multiply by 2: 4
    Now enter the number you want to divide by: 4
    ...
    ```

<small>*[Command Manual](https://www.jlab.org/hpc/PBS/qsub.html)*</small>


### `$ qstat <job_id>`

Displays the human-readable status of a job.

It can be more convenient to always use the `-H` flag.

Common Job States:
* `Q` |	Queued
* `R` |	Running
* `E` |	Exiting
* `F` | Finished

Uncommon Job States:
* `H` |	 Held
* `T` |	 Being transported to a different location (unlikely)
* `W` |	 Waiting for its execution time (if you specified a set start datetime)

*If in an uncommon state (unintentionally) for substantial time, consider reaching out to CRCD.*

Example:
```bash
$ qstat -H 18769

cm:
                                                            Req'd  Req'd   Elap
Job ID          Username Queue    Jobname    SessID NDS TSK Memory Time  S Time
--------------- -------- -------- ---------- ------ --- --- ------ ----- - -----
18769.cm        z1994244 short    ml_retrai* 35427*   1  16   64gb 00:15 F 00:04
```

Common Options:
* `-f` | Prints all available data on a job.
* `-x` | Prints even if the job is historical.
* `-H` | Prints even if the job is historical (with a little bit more data).


<small>*[Command Manual](https://www.jlab.org/hpc/PBS/qstat.html)*</small>

### `$ qdel <job_id>`

Deletes a job by either cancelling or killing it.

Notes:

* This effect is achieved by sending a `SIGTERM` signal to your program, followed by a `SIGKILL` signal. If you want to plan ahead for graceful shutdowns, write your program to intercept these.

Example:
```bash
$ qdel 18769
```

Common Options:
* `-W <seconds>`

    The number of seconds between the `SIGTERM` and `SIGKILL` signals.

    For example, if you have a program which can handle graceful shutdown in about ~5 seconds, but also want to eventually force kill it after 10:
    ```bash
    $ qdel -W 10 18769
    ```

<small>*[Command Manual](https://www.jlab.org/hpc/PBS/qdel.html)*</small>

### `$ qhist`

Prints the history of all batch jobs.

```bash
$ qhist
9646  person_1   project_1       short           1     1    1.0     127  2024/04/05 14:43:20       0:03  2024/04/05 14:43:23       30:01
9648  person_2   project_2       short           2    16    8.0       0  2024/04/05 14:43:44       0:07  2024/04/05 14:43:51       15:00
...
```

Notes:

* Not very useful by itself due to the slew of output, but when paired with `grep` or `awk`, you can filter output by user, project, date, etc to only list what you're looking for.

    Example:
    ```bash
    $ qhist | grep z1994244
    9699  z1994244   zwlab       short           1     1    1.0     127  2024/04/05 14:43:20       0:03  2024/04/05 14:43:23       30:01
    9733  z1994244   zwlab       short           2    16    8.0       0  2024/04/05 14:43:44       0:07  2024/04/05 14:43:51       15:00
    ...
    ```

Common Options:
* `-D|--dates <mm/dd/yyyy[-mm/dd/yyyy] | today | week | month | quarter | year>`

    Filters by a time range.

    Example:
    ```bash
    $ qhist -D year
    18768  person_1   project_1       short        1    16   16.0       0  2024/08/19 16:42:41       5:54  2024/08/19 16:48:35       15:00
    18769  person_2   project_2       short        1    16   16.0       0  2024/08/19 17:06:59       4:38  2024/08/19 17:11:37       15:00
    ...
    ```

<small>*Command Manual: `$ qhist --help`*</small>

## Other Commands

These are commands which aren't as useful as the above four, but do exist and have potential applications, so they are here.

Commands tagged with "❗" are potentially pointless or ineffective on Metis.

### `$ qsig <job_id>`

Sends a signal to a job. By default, this is the `SIGTERM` signal.

Common Options:
* `-s signal`

    Specifies the signal.

    Supports integer representation (such as `9`), or the string name with/without `SIG` (`SIGKILL` or `KILL`)

<small>*[Command Manual](https://www.jlab.org/hpc/PBS/qsig.html)*</small>

### `$ qmsg <job_id>`

Writes a message to one or more output files of a job for the purpose of leaving informative comments.

<small>*[Command Manual](https://www.jlab.org/hpc/PBS/qmsg.html)*</small>

### `$ qalter <job_id>`

Alters an attribute of a job.

<small>*[Command Manual](https://www.jlab.org/hpc/PBS/qalter.html)*</small>

### `$ qmove <destination> <job_id>`

❗ Moves a job from its queue to another destination.

*The potential destinations on Metis are unclear.*

<small>*[Command Manual](https://www.jlab.org/hpc/PBS/qmove.html)*</small>

### `$ qhold <job_id>`

❗ Places a request to 'hold' a job. 

*It's unclear if Metis supports checkpointing. If you believe this command would be helpful, contact CRCD for additional information.*

<small>*[Command Manual](https://www.jlab.org/hpc/PBS/qhold.html)*</small>

### `$ qrls <job_id>` 

❗ Releases the hold on a job.

*It's unclear if Metis supports checkpointing. If you believe this command would be helpful, contact CRCD for additional information.*

<small>*[Command Manual](https://www.jlab.org/hpc/PBS/qrls.html)*</small>

### `$ qselect`

❗ Lists job IDs matching a certain criteria.

*Appears to be entirely broken. Use the `qhist` with `grep` or `awk` to filter output instead.*

<small>*[Command Manual](https://www.jlab.org/hpc/PBS/qselect.html)*</small>