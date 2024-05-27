use colored::*;
use std::process::Command;

pub fn generate_output(mutant_dir:&String){
    let mutant_vec = mutant_dir.split("/").collect::<Vec<&str>>();
    let mutant_num = mutant_vec[mutant_vec.len() - 1];
    let out_file_path = format!("./beskar_out/outfile{}.txt", mutant_num);

    let output = Command::new("grep")
        .args(["PASS", out_file_path.as_str()])
        .output()
        .expect("failed to grep");

    let final_op = String::from_utf8_lossy(&output.stdout);
    let final_op_vec = final_op.split("[PASS]").collect::<Vec<&str>>();
    if final_op == ""{
        println!("{} {}", "[PASS] mutant number".green(), mutant_num.green());
    } else {
        println!("{} {}","[FAIL] mutant number".red(), mutant_num.red());
        println!("{}", "Passing tests:".red());
        for i in 0..final_op_vec.len(){
            let passed_test = final_op_vec[i];
            println!("{}", passed_test.red());
        }
    }
}