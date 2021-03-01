extern crate tokio;
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The usage is similar as with the standard library's `Command` type
    let mut child = Command::new("echo")
        .arg("hello")
        .arg("world")
        .spawn()
        .expect("failed to spawn");

    // Await until the command completes
    let status = child.wait().await?;
    println!("the command exited with: {}", status);
    Ok(())
}
