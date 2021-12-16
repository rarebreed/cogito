use std::process::Stdio;
use std::process::exit;
use tokio::{
    process::Command,
    io::{BufReader, AsyncBufReadExt}
};
use structopt::StructOpt;

struct CmdrOpt {

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("iostat");
    let command = cmd.args(vec!["2", "3"])
        .kill_on_drop(true)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped());

    let mut child = command.spawn()?;

    // Take out the stdout handle so we can read from it
    let stdout = match child.stdout.take() {
        Some(out) => out,
        None => {
            println!("No stdout for child process.");
            exit(-1)
        }
    };
    let mut reader = BufReader::new(stdout).lines();

    // Ensure the child process is spawned in the runtime so it can
    // make progress on its own while we await for any output.
    tokio::spawn(async move {
        let status = child.wait().await.expect("");

        println!("child status was: {}", status);
    });

    while let Some(line) = reader.next_line().await? {
        println!("{}", line);
    }

    Ok(())
}