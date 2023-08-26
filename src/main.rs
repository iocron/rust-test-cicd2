use clap::Parser; // use clap::*;
use glob::glob;
use regex::Regex;
use rust_string_replacer::*;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()] // Required
    string_search: String,

    #[arg()] // Required
    string_replace: String,

    #[arg()] // Required
    file_or_path: String,

    #[arg(short, long, help = "Enable dry run (simulate/don't write files)")]
    dry_run: bool,

    #[arg(short, long, help = "Enable regex for string search/replacement")]
    regex: bool,

    #[arg(short, long, help = "Enable verbose debugging")]
    verbose: bool,

    #[arg(
        short,
        long,
        help = "Disable \"file_or_path\" and use a string input instead."
    )]
    no_file_or_path: bool,
}

fn main() {
    let mut file_modified_count = 0;
    let args = Args::parse();

    if args.file_or_path == "/" {
        panic_red!(
            "Error: Path \"/\" can be very unstable! Please consider choosing a more precise path."
        );
    }

    if args.no_file_or_path {
        // Replace string in string input
        println_yellow!(
            "Replacing string \"{}\" with \"{}\" in string \"{}\"",
            &args.string_search,
            &args.string_replace,
            &args.file_or_path
        );

        // Replace a direct string input
        let text_content_replaced = string_replace(
            &args.string_search,
            &args.string_replace,
            &args.file_or_path,
            args.regex,
        );

        println!("{}", &text_content_replaced);
    } else {
        // Replace strings in path / files
        println_yellow!(
            "Replacing string \"{}\" with \"{}\" in path / files \"{}\"",
            &args.string_search,
            &args.string_replace,
            &args.file_or_path
        );

        // Check if path is a valid file or directory
        let path_parent_without_glob = path_without_glob(&args.file_or_path);
        if !Path::new(&path_parent_without_glob).exists() {
            panic_red!(
                "The argument value does not lead to a valid file or path: {}\n(path validation on: {})",
                args.file_or_path,
                &path_parent_without_glob
            );
        }

        // Glob through files and replace strings
        for file in glob(&args.file_or_path).expect("Failed to read glob pattern") {
            match file {
                Ok(path) => {
                    let file_content = match fs::read_to_string(&path) {
                        Ok(content) => content,
                        Err(err) => {
                            // panic_red!("Error reading file {:?}. {:?}", path, err);
                            eprintln_red!("Error reading file {:?}. {:?}", path, err);
                            continue;
                        }
                    };

                    let file_content_replaced = string_replace(
                        &args.string_search,
                        &args.string_replace,
                        &file_content,
                        args.regex,
                    );

                    if args.verbose {
                        println!("File {:?} content bef: {:?}", &path, &file_content);
                        println!("File {:?} content aft: {:?}", &path, &file_content_replaced);
                    }

                    if file_content != file_content_replaced {
                        if args.dry_run {
                            println_green!("[DRY_RUN] Replaced string in file {:?}", &path);
                        } else {
                            match fs::write(&path, &file_content_replaced) {
                                Ok(_) => println_green!("Replaced string in file {:?}", &path),
                                Err(err) => println_red!("Error on writing to file {:?}", err),
                            };
                        }

                        file_modified_count += 1;
                    }
                }
                Err(err) => println_red!("{:?}", err),
            };

            if file_modified_count == 0 {
                println!("No files/strings have been modified");
            }
        }
    }
}

fn string_replace(
    string_search: &str,
    string_replace: &str,
    string_content: &str,
    regex: bool,
) -> String {
    if regex {
        let re = Regex::new(string_search).unwrap();
        re.replace_all(string_content, string_replace).to_string()
    } else {
        string_content.replace(string_search, string_replace)
    }
}

// Returns: /root/home/**/*.txt => /root/home
// Returns: /root/home/test.txt => /root/home/test.txt
fn path_without_glob(path: &str) -> String {
    let path_separator = std::path::MAIN_SEPARATOR_STR;
    let split = path.split_terminator(path_separator);
    let mut vec: Vec<&str> = vec![];

    for s in split.clone() {
        if !s.contains('*') {
            vec.push(s);
        }
    }

    let result = vec.join(path_separator);

    // println!("path: {:?}", path);
    // println!("split: {:?}", split);
    // println!("result: {:?}", result);

    result
}
