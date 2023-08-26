use serial_test::*;
use std::fs;
use std::process::Command;

fn create_test_files() {
    let files = [
        "./tests/example_files/test1.txt",
        "./tests/example_files/test2.txt",
        "./tests/example_files/folder1/test3.txt",
        "./tests/example_files/folder2/folder2inner/test4.txt",
    ];

    fs::write(
        files[0],
        "UTI\nREPLACE_PENDING=abc\neorwpREPLACE_PENDINGei\nii",
    )
    .expect("File ./tests/test1.txt should have been created.");
    fs::write(files[1], "456").expect("File ./tests/test2.txt should have been created.");
    fs::write(
        files[2],
        "Some weird \nText and REPLACE_PENDING\nis not game over.",
    )
    .expect("File ./tests/test3.txt should have been created.");
    fs::write(files[3], "Bla\nSome REPLACE_PENDING\nshow me.")
        .expect("File ./tests/test4.txt should have been created.");
}

fn count_occurrences(text_input: &str, search_string: &str) -> usize {
    let mut count = 0;
    let mut start = 0;

    while let Some(pos) = text_input[start..].find(search_string) {
        count += 1;
        start += pos + search_string.len();
    }

    count
}

#[test]
#[serial]
fn check_if_arg_string_input_is_working() {
    create_test_files();

    Command::new("cargo")
        .arg("run")
        .arg("REPLACE_PENDING")
        .arg("REPLACE_FINISHED")
        .arg("This is some regular text REPLACE_PENDING instead of a file.")
        .arg("--string-input")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

#[test]
#[serial]
fn check_if_arg_regex_is_working() {
    create_test_files();

    Command::new("cargo")
        .arg("run")
        .arg("(^|\\n)(REPLACE_PENDING)(=.*)")
        .arg("$1 - REPLACE_FINISHED - $3")
        .arg("./tests/example_files/test1.txt")
        .arg("--regex")
        .arg("--verbose")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    let file_content = fs::read_to_string("./tests/example_files/test1.txt").unwrap();
    let occurrences = count_occurrences(&file_content, "REPLACE_FINISHED");

    // The replaced text should have been applied exactly 2 times in test1.txt
    assert_eq!(occurrences, 1);
}

#[test]
#[serial]
fn check_if_arg_dry_run_is_working() {
    create_test_files();

    Command::new("cargo")
        .arg("run")
        .arg("REPLACE_PENDING")
        .arg("REPLACE_FINISHED")
        .arg("./tests/example_files/**/*.txt")
        .arg("--dry-run")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

#[test]
#[serial]
fn check_if_replace_string_works_in_single_file() {
    create_test_files();

    Command::new("cargo")
        .arg("run")
        .arg("REPLACE_PENDING")
        .arg("REPLACE_FINISHED")
        .arg("./tests/example_files/test1.txt")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    let file_content = fs::read_to_string("./tests/example_files/test1.txt").unwrap();
    let occurrences = count_occurrences(&file_content, "REPLACE_FINISHED");

    // The replaced text should have been applied exactly 2 times in test1.txt
    assert_eq!(occurrences, 2);
}

#[test]
#[serial]
fn check_if_replace_string_works_in_files_recursive() {
    create_test_files();

    Command::new("cargo")
        .arg("run")
        .arg("REPLACE_PENDING")
        .arg("REPLACE_FINISHED")
        .arg("./tests/example_files/**/*.txt")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
