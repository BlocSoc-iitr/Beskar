pub fn mutate(path : String, tmp_file_name: String){
    let new_file = PathBuf::from(path.unwrap().path());
    let file_name = new_file.file_name().unwrap().to_str().unwrap();
    let file_path = format!("./src/{}", file_name);
    if file_name.ends_with(".sol") {
        let output = Command::new("gambit")
            .args(["mutate", "--filename", file_path.as_str()])
            .output()
            .expect("failed to execute process");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    
        // let tmp_file_name = format!("./src/{}", "tmp.sol");
        let _ = File::create(&tmp_file_name).unwrap();
        let _ = fs::copy(Path::new(&file_path), Path::new(&tmp_file_name));
        let mutants = fs::read_dir("./gambit_out/mutants").unwrap();
    }
}