use std::process::{
    ExitStatus,
    Stdio,
};
use tokio::{
    io::{
        AsyncBufReadExt,
        BufReader,
    },
    process::Command,
};

pub async fn run(cmd: &str, args: &[&str]) -> anyhow::Result<ExitStatus> {
    // The usage is similar as with the standard library's `Command` type
    let mut cmd = Command::new(cmd);

    // We can't just chain off the .args and .stdout, or rustc will complain that we have moved out of a borrow
    cmd.args(args).stdout(Stdio::piped());

    // Spawn will return immediately
    let mut child = cmd.spawn().expect("failed to spawn subprocess");

    // We need to `take` stdout
    let stdout = child.stdout.take().expect("child did not have a handle to stdout");

    let mut reader = BufReader::new(stdout).lines();

    // spawn the child on a task so we can read while it processes
    let status = tokio::spawn(async move {
        let status = child.wait().await.expect("child process encountered an error");

        println!("child status was: {}", status);
        status
    });

    // meanwhile, in the main task, read what's in the reader (that is being filled by the child process from the other
    // task that is still running)
    while let Some(line) = reader.next_line().await? {
        println!(">: {}", line);
    }

    let status = status.await.unwrap();
    Ok(status)
}
