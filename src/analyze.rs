mod helpers;

use serde::{Serialize, Deserialize};
use std::io::{BufRead, BufReader};
use walkdir:: WalkDir;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    pub files: Vec<String>,
    pub num_files: usize,
    pub total_lines: usize,
    pub project_type: String,
}

pub fn analyze_project(path: &str) -> Option<Project> {

    use std::fs::File;
    let file_lines: Vec::<String>;
    let extension: String; 
    let start_path: String;
    let project_type: &str;
    
    if fs::metadata(path).is_ok() {

        if helpers::is_flutter(path) {
            start_path = format!("{}/lib", path);
            extension = ".dart".to_string();
            project_type = "flutter";

        } else if helpers::is_rust(path) {
            extension = ".rs".to_string();
            start_path = format!("{}/src", path);
            project_type = "rust";
        }else if helpers::is_android(path) {
            extension = ".java".to_string();
            start_path = format!("{}/app/src/main/java", path); 
            project_type = "android";
        }  else if helpers::is_webapp(path) {
            extension = ".js".to_string();
            start_path = path.to_string();
            project_type = "web";
        } else {
            extension = ".js".to_string();
            start_path = format!("{}/src", path);
            project_type = "none";
        } 

        file_lines = find_files_with_extension(&extension, &start_path);

        let mut sum = 0;
        for file in &file_lines {
            let file = File::open(file).expect("unable to open file");
            let reader = BufReader::new(file); 
            let line_count = reader.lines().count();
            sum += line_count;
        }


        Some(Project{
            files: file_lines.clone(),
            num_files: file_lines.len(),
            total_lines: sum,
            project_type: project_type.to_string(),
        })
    } else {
        None
    }
}
// find files with given extension
fn find_files_with_extension(extension: &str, path: &str) -> Vec<String> {

    let mut found_files = Vec::new();
    
    let walker = WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| !helpers::is_hidden(e) && !helpers::is_build_or_test(e)) {
        if let Ok(entry) = entry {
            
            if let Some(file_path) = entry.path().to_str() {
                if file_path.ends_with(extension) {//&& is_recent(file_path) {

                    found_files.push(file_path.to_string());
                }
            }
        }
    }

    found_files
}