use clap::{App, Arg};
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use std::io::{BufRead, BufReader};
use walkdir::{WalkDir, DirEntry};

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    files: Vec<String>,
    num_files: usize,
    total_lines: usize,
    project_type: String,
}

fn analyze_project(path: &str) -> Option<Project> {

    use std::fs::File;
    let file_lines: Vec::<String>;
    let extension: String; 
    let start_path: String;
    let project_type: &str;
    
    if fs::metadata(path).is_ok() {

        if is_flutter(path) {
            start_path = format!("{}/lib", path);
            extension = ".dart".to_string();
            project_type = "flutter";

        } else if is_rust(path) {
            extension = ".rs".to_string();
            start_path = format!("{}/src", path);
            project_type = "rust";
        }else if is_android(path) {
            extension = ".java".to_string();
            start_path = format!("{}/app/src/main/java", path); 
            project_type = "android";
        }  else if is_webapp(path) {
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
    for entry in walker.filter_entry(|e| !is_hidden(e) && !is_build_or_test(e)) {
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
// function to detect hidden directories (you most likely won't be working with them)
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

// function to detect build and test directories in projects
fn is_build_or_test(entry: &DirEntry) -> bool { 
    if let Some(file_name) = entry.file_name().to_str() {

        let build_dirs = ["build", "cmake_build", "node_modules", "target"];
        let test_dirs = vec!["test", "tests", "unit_tests", "__tests__"];

        build_dirs.iter().any(|dir| dir.to_string() == file_name) || test_dirs.iter().any(|dir| dir.to_string() == file_name)
    } else {
        false
    }
}

// the next functions determine which type of project we're dealing with
fn is_flutter(path: &str) -> bool {
   
    let dir = Path::new(path);
    let pubspec = PathBuf::from(dir).join("pubspec.yaml");
    let android = dir.join("android");
    let ios = dir.join("ios");
    let libmain = PathBuf::from(dir).join("lib/main.dart");

    dir.exists() && pubspec.exists() && android.exists() && ios.exists() && libmain.exists()
}

fn is_rust(path: &str) -> bool {
    let dir = Path::new(path);
    let cargo_toml = PathBuf::from(dir).join("Cargo.toml");
    let mainrs = PathBuf::from(dir).join("src/main.rs"); 

    cargo_toml.exists() && mainrs.exists()
}

fn is_android(path: &str) -> bool {
    let dir = Path::new(path);
    let manifest = dir.join("app/src/main/AndroidManifest.xml");
    let gradle = dir.join("app/build.gradle");
    
    manifest.exists() && gradle.exists()
}

fn is_webapp(path: &str) -> bool {
    let dir = Path::new(path);
    let package_json = dir.join("package.json");

    let files = fs::read_dir(path).unwrap();
    let js_ts_files_exist = files
                            .filter_map(|entry| entry.ok())
                            .filter(|e| {
                                let path = e.path();
                                path.is_file() && (path.extension().is_some() && 
                                    (path.extension().unwrap() == "js" ||
                                    path.extension().unwrap() == "ts"     
                                    ))
                            }).count() > 0;

    package_json.exists() || js_ts_files_exist
}

fn main() {

    let app = App::new("ThisManyLines")
    .version("1.0.0")
    .author("logos")
    .about("a cli tool that displays how many lines of code you've written")
    .arg(Arg::with_name("specific")
        .long("specific")
        .short('s')
        .takes_value(true)
        .help("analyzes a specific project folder, given the path (you don't have to specify what project type it is)")
    );

    let matches = app.get_matches();
    if let Some(path) = matches.value_of("specific") {
        match analyze_project(path) {
            Some(project) => {
                println!("\ttype of project: \t\t{}", project.project_type);
                println!("\tnumber of files: \t\t{}", project.num_files);
                println!("\ttotal number of lines: \t\t{}", project.total_lines);
            }
            None => {
                println!("Unable to analyze project");
                // Take appropriate action when the project is None
            }
        }
    } 

}






