use clap::Command;

#[tokio::main]
async fn main() {
    Command::new("docker demo").about("Build an executable for ");
}
