#![allow(unused)]
use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;
use std::io;
use clap::Parser;
use walkdir::WalkDir;

/// Search for a pattern in directory tree that matches file extension
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    file_ext: String,
}

fn is_recent (file_path: &str) -> bool {
    /// determine how many days it was since the file was last modified,
    /// and return true if it's less than a day
    
    let metadata = fs::metadata(file_path).expect("Failed to read metadata");

    let modified = metadata.modified().expect("failed to read modified time");

    let now = SystemTime::now(); 

    let time_difference = now.duration_since(modified).expect("failed to find time difference");

    let days = time_difference.as_secs() / (3600);

    return days <= 24;
}



/// find files with given extension
fn find_files_with_extension(extension: &str) -> Vec<String> {

    let mut found_files = Vec::new();
    
    for entry in WalkDir::new("/home/") {
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

}





