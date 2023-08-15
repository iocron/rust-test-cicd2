use std::process::Command;
use std::fs;
use std::time::Duration;
use std::thread::sleep;

fn create_test_files() {
    let files = [
        "./tests/example_files/test1.txt", 
        "./tests/example_files/test2.txt",
        "./tests/example_files/folder1/test3.txt", 
        "./tests/example_files/folder2/folder2inner/test4.txt" 
    ];

    for file in files {
        fs::remove_file(file).ok();
    }

    fs::write(files[0], "UTI\nREPLACE_PENDINGabc\neorwpREPLACE_PENDINGei\nii").expect("File ./tests/test1.txt should have been created.");
    fs::write(files[1], "456").expect("File ./tests/test2.txt should have been created.");
    fs::write(files[2], "Some weird \nText and REPLACE_PENDING\nis not game over.").expect("File ./tests/test4.txt should have been created.");
    fs::write(files[3], "Bla\nSome REPLACE_PENDING\nshow me.").expect("File ./tests/test4.txt should have been created.");
}

#[test]
fn check_if_replace_string_works_in_single_file() {
    sleep(Duration::from_millis(100)); 
    create_test_files();

    Command::new("cargo")
        .arg("run")
        .arg("REPLACE_PENDING")
        .arg("REPLACE_FINISHED")
        .arg("./tests/example_files/test1.txt")
        .spawn()
        .expect("cargo run failed to spawn.");
}

#[test]
fn check_if_replace_string_works_in_files_recursive() {
    sleep(Duration::from_millis(200));
    create_test_files();

    Command::new("cargo")
        .arg("run")
        .arg("REPLACE_PENDING")
        .arg("REPLACE_FINISHED")
        .arg("./tests/example_files/**/*.txt")
        .spawn()
        .expect("cargo run failed to spawn.");
}