use std::process::Command;
use std::fs::{self};
use std::path::PathBuf;
use std::path::Path;

// Tasks:
// 1. run tests on generated mutants 
// 2. make it a tool

fn main() {
    // assuming run from foundry project root
    let paths = fs::read_dir("./src").unwrap();

    for path in paths {
        let new_file = PathBuf::from(path.unwrap().path());
        let file_name = new_file.file_name().unwrap().to_str().unwrap();
        let file_path = format!("/home/preeti/Beskar/Beskar/helper/{}",file_name);
        let to= Path::new(&file_path);
        if file_name.ends_with(".sol"){
            let _ = fs::copy(new_file, to);
            let output = Command::new("gambit")
                .args(["mutate", "--filename",file_path.as_str()])
                .output()
                .expect("failed to execute process");

            println!("status: {}", output.status);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
}