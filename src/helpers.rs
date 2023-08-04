use std::fs;
use std::path::{Path, PathBuf};

// function to detect hidden directories (you most likely won't be working with them)
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

// function to detect build and test directories in projects
pub fn is_build_or_test(entry: &DirEntry) -> bool { 
    if let Some(file_name) = entry.file_name().to_str() {

        let build_dirs = ["build", "cmake_build", "node_modules", "target"];
        let test_dirs = vec!["test", "tests", "unit_tests", "__tests__"];

        build_dirs.iter().any(|dir| dir.to_string() == file_name) || test_dirs.iter().any(|dir| dir.to_string() == file_name)
    } else {
        false
    }
}

// the next functions determine which type of project we're dealing with
pub fn is_flutter(path: &str) -> bool {
   
    let dir = Path::new(path);
    let pubspec = PathBuf::from(dir).join("pubspec.yaml");
    let android = dir.join("android");
    let ios = dir.join("ios");
    let libmain = PathBuf::from(dir).join("lib/main.dart");

    dir.exists() && pubspec.exists() && android.exists() && ios.exists() && libmain.exists()
}

pub fn is_rust(path: &str) -> bool {
    let dir = Path::new(path);
    let cargo_toml = PathBuf::from(dir).join("Cargo.toml");
    let mainrs = PathBuf::from(dir).join("src/main.rs"); 

    cargo_toml.exists() && mainrs.exists()
}

pub fn is_android(path: &str) -> bool {
    let dir = Path::new(path);
    let manifest = dir.join("app/src/main/AndroidManifest.xml");
    let gradle = dir.join("app/build.gradle");
    
    manifest.exists() && gradle.exists()
}

pub fn is_webapp(path: &str) -> bool {
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

pub fn is_python(path: &str) -> bool {

    let files = fs::read_dir(path).unwrap();
    let python_exists = files
                            .filter_map(|entry| entry.ok())
                            .filter(|e| {
                                let path = e.path();
                                path.is_file() && (path.extension().is_some() && 
                                    (path.extension().unwrap() == "py" ))
                            }).count() > 0;

    python_exists
}
