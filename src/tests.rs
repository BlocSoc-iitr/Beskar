use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

use std::fs::{self, File, create_dir};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub fn run_tests(mutant_dir:&String, mutant_check: &PathBuf, path: &PathBuf ){

    let new_file = PathBuf::from(path);
    let file_name = new_file.file_name().unwrap().to_str().unwrap();
    let file_path = format!("./src/{}", file_name);

    let mutant_vec = mutant_dir.split("/").collect::<Vec<&str>>();
    let mutant_num = mutant_vec[mutant_vec.len() - 1];
    let mutant_file = format!("{}/src/{}", mutant_check.display(), file_name);

    let _ = fs::copy(Path::new(&mutant_file), Path::new(&file_path));

    println!("Mutant Number: {}", mutant_num);
    let mut sp = Spinner::new(Spinners::Dots9, "running tests".into());
    sleep(Duration::from_secs(3));

    let _ = create_dir("./beskar_out");
    let out_file_path = format!("./beskar_out/outfile{}.txt", mutant_num);
    let out_file = File::create(out_file_path.clone()).expect("failed to open output file.");

    let mut child = Command::new("forge")
        .args(["test"])
        .stdout(out_file)
        .spawn()
        .expect("failed to execute forge test");

    let _ = child.wait();
    sp.stop();
    println!();
}