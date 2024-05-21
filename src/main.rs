extern crate rev_buf_reader;

use std::process::Command;
use std::fs::{self, create_dir, File};
use std::path::PathBuf;
use std::path::Path;
use std::io::BufRead;
use rev_buf_reader::RevBufReader;

// Tasks:
// 1. manage report

// 1. grep "Failing tests:" beskar_out/outfile.txt -A 2 > beskar_out/outfile2.txt
// 2. grep "FAIL. "  beskar_out/outfile2.txt

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
            println!("{}", String::from_utf8_lossy(&output.stdout));

            let tmp_file_name = format!("./src/{}","tmp.sol");
            let _ = File::create(&tmp_file_name).unwrap();
            let _ = fs::copy(Path::new(&file_path),Path::new(&tmp_file_name));
            let mutants = fs::read_dir("./gambit_out/mutants").unwrap();

            let mut failed = 0;
            let mut passed = 0;

            for mutant in mutants{
                let mutant_check = mutant.as_ref().unwrap();
                let mutant_dir = mutant.as_ref().unwrap().file_name().into_string().unwrap();
                let mutant_vec  = mutant_dir.split("/").collect::<Vec<&str>>();
                let mutant_num = mutant_vec[mutant_vec.len()-1];
                let mutant_file = format!("{}/src/{}",mutant_check.path().display(), file_name);

                // direct terminal command output to file 
                let _ = fs::copy(Path::new(&mutant_file),Path::new(&file_path));
                let _ = create_dir("./beskar_out");
                let out_file = File::create("./beskar_out/outfile.txt").expect("failed to open output file.");
                let output2 = Command::new("forge")
                .args(["test"])
                .stdout(out_file)
                .spawn()
                .expect("failed to execute forge test");

                let tmp_out = Command::new("forge")
                .args(["test"])
                .output()
                .expect("failed to execute forge test");
                
                if tmp_out.status.success() {
                    passed+=1;
                } else {
                    failed+=1;
                }

                // read output file and read last line
                let outfile = File::open("./beskar_out/outfile.txt").expect("failed to open output file");
                let last_line = lines_from_file(&outfile,1 );
                for i in 0..last_line.len(){
                    println!("{}",last_line[i]);
                }
                println!("mutant number : {} \ntests failed : {} \ntests passed : {}", mutant_num.path().display(), failed, passed);
                let _ = fs::copy(Path::new(&tmp_file_name),Path::new(&file_path));

                let out_file_2 = File::create("./beskar_out/outfile2.txt").expect("failed to open output file.");
                let output3 = Command::new("grep")
                .args(["tests:", "beskar_out/outfile.txt", "-A", "2"])
                .stdout(out_file_2)
                .spawn()
                .expect("failed to execute grep");

                let output4 = Command::new("grep")
                .args(["FAIL. ", "beskar_out/outfile2.txt"])
                .output()
                .expect("failed to execute grep");

                println!("{}",String::from_utf8_lossy(&output.stdout));
                // let _ = fs::remove_file(Path::new(&tmp_file_name));
                // let _ = fs::remove_dir(Path::new("./gambit_out/"));
            }
        }
    }

}