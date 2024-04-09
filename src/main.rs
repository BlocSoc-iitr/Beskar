use std::process::Command;
use std::fs::{self, File};
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
        let file_path = format!("./src/{}",file_name);
        if file_name.ends_with(".sol"){
            let output = Command::new("gambit")
                .args(["mutate", "--filename",file_path.as_str()])
                .output()
                .expect("failed to execute process");

            println!("status: {}", output.status);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

            let tmp_file_name = format!("./src/{}","tmp.sol");
            let _ = File::create(&tmp_file_name).unwrap();
            let _ = fs::copy(Path::new(&file_path),Path::new(&tmp_file_name));
            let mutants = fs::read_dir("./gambit_out/mutants").unwrap();

            for mutant in mutants{
                let mutant_num = mutant.unwrap();
                let mutant_file = format!("{}/src/{}",mutant_num.path().display(), file_name);
                
                let _ = fs::copy(Path::new(&mutant_file),Path::new(&file_path));
                let output2 = Command::new("forge")
                .args(["test"])
                .output()
                .expect("failed to execute forge test");
                println!("status: {}", output2.status);
                println!("stdout: {}", String::from_utf8_lossy(&output2.stdout));
                println!("stderr: {}", String::from_utf8_lossy(&output2.stderr));

                let _ = fs::copy(Path::new(&tmp_file_name),Path::new(&file_path));
                let _ = fs::remove_file(Path::new(&tmp_file_name));
                let _ = fs::remove_dir("./gambit_out");
            }
            
        }
    }

}