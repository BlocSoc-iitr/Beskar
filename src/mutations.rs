use std::fs::{self, File};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub fn mutate(path : &PathBuf, tmp_file_name: &String) {
    let new_file = PathBuf::from(path);
    let file_name = new_file.file_name().unwrap().to_str().unwrap();
    let file_path = format!("./src/{}", file_name);

    if file_name.ends_with(".sol") {
        let output = Command::new("gambit")
            .args(["mutate", "--filename", file_path.as_str()])
            .output()
            .expect("failed to execute process");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    
        let _ = File::create(&tmp_file_name).unwrap();
        let _ = fs::copy(Path::new(&file_path), Path::new(&tmp_file_name));
    }
}