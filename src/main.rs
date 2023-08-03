#![allow(unused)]
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use std::io::{BufRead, BufReader};
use clap::Parser;
use walkdir::{DirEntry, WalkDir};

/// Search for a pattern in directory tree that matches file extension
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    file_ext: String,
    path: String,
}

fn is_recent (file_path: &str) -> bool {
    /// determine how many days it was since the file was last modified,
    /// and return true if it's less than a day
    
    let metadata = fs::metadata(file_path).expect("Failed to read metadata");
    let modified = metadata.modified().expect("failed to read modified time");
    let now = SystemTime::now(); 
    let time_difference = now.duration_since(modified).expect("failed to find time difference");
    let days = time_difference.as_secs() / (3600 * 24);

    return days <= 300;
}

/// function to detect hidden directories (you most likely won't be working with them)
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

/// function to detect build and test directories in projects
fn is_build_or_test(entry: &DirEntry) -> bool { 
    if let Some(file_name) = entry.file_name().to_str() {

        let build_dirs = ["build", "cmake_build", "node_modules", "target"];
        let test_dirs = vec!["test", "tests", "unit_tests", "__tests__"];

        build_dirs.iter().any(|dir| dir.to_string() == file_name) || test_dirs.iter().any(|dir| dir.to_string() == file_name)
    } else {
        false
    }
}

fn is_flutter(path: &str) -> bool {
    let pubspec_path = format!("{}/pubspec.yaml", path);
    let android_path = format!("{}/android", path);
    let ios_path = format!("{}/ios", path);
    let lib_path = format!("{}/lib", path);
    if fs::metadata(pubspec_path.as_str()).is_ok() &&
            fs::metadata(android_path.as_str()).is_ok() && 
            fs::metadata(ios_path.as_str()).is_ok() &&
            fs::metadata(lib_path.as_str()).is_ok() {
                true
            } else {
               false 
            }

}
fn analyze_project(path: &str, ext: &str) -> i32 {

    use std::fs::File;
    if fs::metadata(path).is_ok() {

    /// if these conditions hold true, project is flutter.
        if is_flutter(path) {

            let lib_path = format!("{}/lib", path);
            let file_lines = find_files_with_extension(&ext, &lib_path); 
            println!("path: {}", lib_path);

            let mut sum = 0;
            for file in file_lines {
                println!("file to be read: {}", file);
                let file = File::open(file).expect("unable to open file");
                let reader = BufReader::new(file); 
                let line_count = reader.lines().count();
                sum += line_count;
            }

            sum.try_into().unwrap()
        } else {
            11111
        }   
    } else {
        22222
    }
}
/// find files with given extension
fn find_files_with_extension(extension: &str, path: &str) -> Vec<String> {

    let mut found_files = Vec::new();
    
    let walker = WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e) && !is_build_or_test(e)) {
        if let Ok(entry) = entry {
            
            if let Some(file_path) = entry.path().to_str() {
                if file_path.ends_with(extension) && is_recent(file_path) {
                    found_files.push(file_path.to_string());
                }
            }
        }
    }

    found_files
}
fn main() {
    /// store possible file extensions in hash map
    /*
    let mut file_extensions: HashMap<&str, &str> = HashMap::new();

    file_extensions.insert("cpp", ".cpp");
    file_extensions.insert("java", ".java");
    file_extensions.insert("python", ".py");
    file_extensions.insert("dart", ".dart");
    file_extensions.insert("rust", ".rs");
    
    let arg = Cli::parse();
    let extension = file_extensions.get(arg.file_ext.as_str());  

    
    let files = find_files_with_extension(&extension.unwrap());

    for file in files {
        println!("{}", file);
    }
*/

    let arg = Cli::parse();

    println!("{}", analyze_project(&arg.path, &arg.file_ext));   
}





