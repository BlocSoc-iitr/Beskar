
use std::process::Command;

fn main() {
    let output = Command::new("gambit")
        .args(["mutate", "--json","/home/preeti/Beskar/Beskar/helper/gambit_config/gambit_config.js"])
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

}
