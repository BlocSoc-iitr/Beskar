use std::fs::{self};
use std::path::Path;
use std::path::PathBuf;

use Beskar::{generate_output, mutate, run_tests};

/// 1. terminal report overview
/*
----------------------------------------------
|                                            |
| mutant  number: **mutant number**          |
| running tests(processing)                  |
| testing completed                          |
| if passed: display [PASS] with green color |
| else: [FAIL] with reason in red color      |
| reason: passing tests info                 |
|                                            |
----------------------------------------------
 */

fn main() {
    /* assuming run from foundry project root */

    let args: Vec<String> = std::env::args().collect();
    match args[1].as_str(){
        "run" => {
            let paths = fs::read_dir("./src").unwrap();

            for path_ in paths {
                let tmp_file_name = format!("./src/{}", "tmp.sol");
                let path = path_.unwrap().path();
                let new_file = PathBuf::from(path.clone());
                let file_name = new_file.file_name().unwrap().to_str().unwrap();
                let file_path = format!("./src/{}", file_name);

                mutate(&path, &tmp_file_name);
                let mutants = fs::read_dir("./gambit_out/mutants").unwrap();
                for mutant in mutants {
                    let mutant_check = mutant.as_ref().unwrap().path();
                    let mutant_dir = mutant.as_ref().unwrap().file_name().into_string().unwrap();
                    run_tests(&mutant_dir, &mutant_check, &path);
                    generate_output(&mutant_dir)
                }
                let _ = fs::copy(Path::new(&tmp_file_name),Path::new(&file_path));

            }
        }
        _ => {
            println!("invalid command");
        }

    }
}
