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