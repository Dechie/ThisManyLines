mod analyze;

use clap::{App, Arg};

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
        match analyze::analyze_project(path) {
            Some(project) => {
                println!("number of files: {}", project.num_files);
                println!("total number of lines: {}", project.total_lines);
            }
            None => {
                println!("Unable to analyze project");
                // Take appropriate action when the project is None
            }
        }
    }

}






