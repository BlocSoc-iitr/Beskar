/* */
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use ctrlc;
use regex::Regex;

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


fn restore_original_files(original_files: &Vec<(PathBuf, String)>) {
    for (path, content) in original_files {
        if let Err(e) = fs::write(path, content) {
            eprintln!("Failed to restore file {:?}: {}", path, e);
        }
    }
}

fn delete_matching_folders() -> Result<(), Box<dyn std::error::Error>> {
    // Parent directory where the gambit_outX folders exist
    let parent_dir = Path::new(".");

    // Regex pattern to match folder names like "gambit_out1", "gambit_out2", etc.
    let re = Regex::new(r"^gambit_out\d+$")?;

    // Iterate through the directory entries
    for entry in fs::read_dir(parent_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a directory
        if path.is_dir() {
            // Extract the directory name as a string
            if let Some(folder_name) = path.file_name().and_then(|name| name.to_str()) {
                // Check if the folder name matches the regex
                if re.is_match(folder_name) {
                    // Remove the directory
                    println!("Deleting directory: {}", folder_name);
                    fs::remove_dir_all(&path)?;
                }
            }
        }
    }

    Ok(())
}

fn process_contract(
    path: PathBuf,
    index: usize,
    running: Arc<AtomicBool>,
    original_files: Arc<Mutex<Vec<(PathBuf, String)>>>
) {
    let new_file = PathBuf::from(path.clone());
    let file_name = new_file.file_name().unwrap().to_str().unwrap();
    if !file_name.ends_with(".sol") {
        return;
    }

    let file_path = format!("./src/{}", file_name);

    // Store original content
    if let Ok(content) = fs::read_to_string(&file_path) {
        original_files.lock().unwrap().push((PathBuf::from(&file_path), content));
    }

    let tmp_file_name = format!("./src/{}.tmp", file_name);
    
    mutate(&path, &tmp_file_name);

    let new_name_gambit = format!("gambit_out{}", index);
    match fs::rename("gambit_out", &new_name_gambit) {
        Ok(_) => println!("Directory renamed from '{}' to '{}'.", "gambit_out", new_name_gambit),
        Err(e) => eprintln!("in match: Error renaming directory: {}", e),
    }

    let new_name_mutant = format!("gambit_out{}/mutants", index);
    let mutants = fs::read_dir(&new_name_mutant).unwrap();

    for mutant in mutants {
        if !running.load(Ordering::SeqCst) {
            break;
        }
        let mutant_check = mutant.as_ref().unwrap().path();
        let mutant_dir = mutant.as_ref().unwrap().file_name().into_string().unwrap();
        
        run_tests(&mutant_dir, &mutant_check, &path, &new_name_gambit,index);
        generate_output(&mutant_dir, &new_name_gambit);
    }

    // Restore original file
    if let Err(e) = fs::copy(Path::new(&tmp_file_name), Path::new(&file_path)) {
        eprintln!("Failed to restore original file {}: {}", file_path, e);
    }
    if let Err(e) = fs::remove_file(&tmp_file_name) {
        eprintln!("Failed to remove temporary file {}: {}", tmp_file_name, e);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 || args[1] != "run" {
        println!("Usage: beskar run");
        return;
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    match delete_matching_folders() {
        Ok(_) => println!("All matching directories deleted."),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Set up the Ctrl+C handler
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let original_files = Arc::new(Mutex::new(Vec::new()));

    let paths = fs::read_dir("./src").unwrap();
    
    let mut handles = vec![];

    for (index, path_) in paths.enumerate() {
        if !running.load(Ordering::SeqCst) {
            break;
        }

        let path = path_.unwrap().path();
        let running_clone = Arc::clone(&running);
        let original_files_clone = Arc::clone(&original_files);

        let handle = thread::spawn(move || {
            process_contract(path, index, running_clone, original_files_clone);
        });//it is not possible to have 2 folders of the same name in the same directory 

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // If execution was interrupted, restore all original files
    if !running.load(Ordering::SeqCst) {
        println!("\nExecution interrupted. Restoring original files...");
        restore_original_files(&original_files.lock().unwrap());
    }

    println!("Execution completed.");
}

/*

use std::fs::{self};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use ctrlc;
use regex::Regex;

use Beskar::{generate_output, mutate, run_tests};

fn restore_original_files(original_files: &Vec<(PathBuf, String)>) {
    for (path, content) in original_files {
        if let Err(e) = fs::write(path, content) {
            eprintln!("Failed to restore file {:?}: {}", path, e);
        }
    }
}

fn delete_matching_folders() -> Result<(), Box<dyn std::error::Error>> {
    let parent_dir = Path::new(".");
    let re = Regex::new(r"^gambit_out\d+$")?;

    for entry in fs::read_dir(parent_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(folder_name) = path.file_name().and_then(|name| name.to_str()) {
                if re.is_match(folder_name) {
                    println!("Deleting directory: {}", folder_name);
                    fs::remove_dir_all(&path)?;
                }
            }
        }
    }

    Ok(())
}

fn process_contract(
    path: PathBuf,
    index: usize,
    running: Arc<AtomicBool>,
    original_files: Arc<Mutex<Vec<(PathBuf, String)>>>,
    rename_sync: Arc<(Mutex<bool>, Condvar)>
) {
    let new_file = PathBuf::from(path.clone());
    let file_name = new_file.file_name().unwrap().to_str().unwrap();
    if !file_name.ends_with(".sol") {
        return;
    }

    let file_path = format!("./src/{}", file_name);

    if let Ok(content) = fs::read_to_string(&file_path) {
        original_files.lock().unwrap().push((PathBuf::from(&file_path), content));
    }

    let tmp_file_name = format!("./src/{}.tmp", file_name);
    
    mutate(&path, &tmp_file_name);

    let (lock, cvar) = &*rename_sync;
    let mut renamed = lock.lock().unwrap();

    if !*renamed {
        let new_name_gambit = format!("gambit_out{}", index);
        match fs::rename("gambit_out", &new_name_gambit) {
            Ok(_) => {
                println!("Directory renamed from 'gambit_out' to '{}'.", new_name_gambit);
                *renamed = true;
                cvar.notify_all();  // Notify all threads that the folder renaming is done
            }
            Err(e) => eprintln!("Error renaming directory: {}", e),
        }
    }

    // Wait for renaming to complete if it's not done by this thread
    while !*renamed {
        renamed = cvar.wait(renamed).unwrap();
    }

    let new_name_mutant = format!("gambit_out{}/mutants", index);
    let mutants = fs::read_dir(&new_name_mutant).unwrap();

    for mutant in mutants {
        if !running.load(Ordering::SeqCst) {
            break;
        }
        let mutant_check = mutant.as_ref().unwrap().path();
        let mutant_dir = mutant.as_ref().unwrap().file_name().into_string().unwrap();
        
        run_tests(&mutant_dir, &mutant_check, &path, &new_name_gambit);
        generate_output(&mutant_dir, &new_name_gambit);
    }

    if let Err(e) = fs::copy(Path::new(&tmp_file_name), Path::new(&file_path)) {
        eprintln!("Failed to restore original file {}: {}", file_path, e);
    }
    if let Err(e) = fs::remove_file(&tmp_file_name) {
        eprintln!("Failed to remove temporary file {}: {}", tmp_file_name, e);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 || args[1] != "run" {
        println!("Usage: beskar run");
        return;
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    match delete_matching_folders() {
        Ok(_) => println!("All matching directories deleted."),
        Err(e) => eprintln!("Error: {}", e),
    }

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let original_files = Arc::new(Mutex::new(Vec::new()));

    // Mutex and Condvar for renaming synchronization
    let rename_sync = Arc::new((Mutex::new(false), Condvar::new()));

    let paths = fs::read_dir("./src").unwrap();
    let mut handles = vec![];

    for (index, path_) in paths.enumerate() {
        if !running.load(Ordering::SeqCst) {
            break;
        }

        let path = path_.unwrap().path();
        let running_clone = Arc::clone(&running);
        let original_files_clone = Arc::clone(&original_files);
        let rename_sync_clone = Arc::clone(&rename_sync);

        let handle = thread::spawn(move || {
            process_contract(path, index, running_clone, original_files_clone, rename_sync_clone);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    if !running.load(Ordering::SeqCst) {
        println!("\nExecution interrupted. Restoring original files...");
        restore_original_files(&original_files.lock().unwrap());
    }

    println!("Execution completed.");
}
*/