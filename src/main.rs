use clap::Parser;
use glob::glob;
use std::fs;
use rust_string_replacer::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // #[arg(short, long)]
    // optional_argument: String,
    #[arg()]
    string_search: String,

    #[arg()]
    string_replace: String,

    #[arg()]
    file_or_path: String,
}

fn main() {
    let mut file_modified_count = 0;
    let args = Args::parse();

    if args.file_or_path == "/" {
        panic_red!("Error: Path \"/\" can be very unstable! Please consider choosing a more precise path.");
    }

    println_yellow!(
        "Replacing string \"{}\" with \"{}\" in file/folder \"{}\"", 
        &args.string_search, &args.string_replace, &args.file_or_path
    );

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

                let file_content_replaced = file_content.replace(&args.string_search, &args.string_replace);

                // println!("File {:?} content bef: {:?}", &path, file_content);
                // println!("File {:?} content aft: {:?}", &path, file_content_replaced);

                if file_content != file_content_replaced {
                    match fs::write(&path, &file_content_replaced) {
                        Ok(_) => println_green!("Replaced string in file {:?}", &path),
                        Err(err) => println_red!("Error on writing to file {:?}", err),
                    };

                    file_modified_count += 1;
                }
            }
            Err(err) => println!("{:?}", err),
        };
    }

    if file_modified_count == 0 {
        println!("No files/strings have been modified");
    }
}
