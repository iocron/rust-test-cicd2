use clap::Parser; // use clap::*;
use glob::glob;
use regex::Regex;
use rust_string_replacer::*;
use std::fs;

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

    if !args.no_file_or_path || (args.no_file_or_path && args.verbose) {
        println_yellow!(
            "Replacing string \"{}\" with \"{}\" in \"{}\"",
            &args.string_search,
            &args.string_replace,
            &args.file_or_path
        );
    }

    if args.no_file_or_path {
        let text_content_replaced = string_replace(
            &args.string_search,
            &args.string_replace,
            &args.file_or_path,
            args.regex,
        );

        println!("{}", &text_content_replaced);
    } else {
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
