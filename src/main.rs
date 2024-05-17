extern crate rev_buf_reader;

use std::process::Command;
use std::fs::{self, create_dir, File};
use std::path::PathBuf;
use std::path::Path;
use std::io::BufRead;
use rev_buf_reader::RevBufReader;

// Tasks:
// 1. run tests on generated mutants 
// 2. make it a tool

fn lines_from_file(file: &File, limit: usize) -> Vec<String> {
    let buf = RevBufReader::new(file);
    buf.lines().take(limit).map(|l| l.expect("Could not parse line")).collect()
}

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
                
                // direct terminal command output to file 
                let _ = fs::copy(Path::new(&mutant_file),Path::new(&file_path));
                let _ = create_dir("./beskar_out");
                let out_file = File::create("./beskar_out/outfile.txt").expect("failed to open output file.");
                let output2 = Command::new("forge")
                .args(["test"])
                .stdout(out_file)
                .spawn()
                .expect("failed to execute forge test");
                // println!("status: {}", output2.status);
                // println!("stdout: {}", String::from_utf8_lossy(&output2.stdout));
                // println!("stderr: {}", String::from_utf8_lossy(&output2.stderr));

                // read output file and read last line
                let outfile = File::open("./beskar_out/outfile.txt").expect("failed to open output file");
                let last_line = lines_from_file(&outfile,1 );
                println!("{}",last_line[0]);

                let _ = fs::copy(Path::new(&tmp_file_name),Path::new(&file_path));
                let _ = fs::remove_file(Path::new(&tmp_file_name));
                let _ = fs::remove_dir("./gambit_out");
            }
            
        }
    }

}