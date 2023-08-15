use std::process::Command;
use std::fs;
use std::time::Duration;
use std::thread::sleep;
use serial_test::*;

fn create_test_files() {
    // Using delay+#[serial] against file_locks/writes/race-conditions
    // => (the combination of sleep+serial_test works against those problems)
    sleep(Duration::from_millis(100));

    let files = [
        "./tests/example_files/test1.txt", 
        "./tests/example_files/test2.txt",
        "./tests/example_files/folder1/test3.txt", 
        "./tests/example_files/folder2/folder2inner/test4.txt" 
    ];

    // for file in files {
    //     // Remove file (if exists), not really needed, because fs::write overwrites by default
    //     fs::remove_file(file).ok();
    // }

    fs::write(files[0], "UTI\nREPLACE_PENDINGabc\neorwpREPLACE_PENDINGei\nii").expect("File ./tests/test1.txt should have been created.");
    fs::write(files[1], "456").expect("File ./tests/test2.txt should have been created.");
    fs::write(files[2], "Some weird \nText and REPLACE_PENDING\nis not game over.").expect("File ./tests/test4.txt should have been created.");
    fs::write(files[3], "Bla\nSome REPLACE_PENDING\nshow me.").expect("File ./tests/test4.txt should have been created.");
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
        .expect("cargo run failed to spawn.");
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
        .expect("cargo run failed to spawn.");
}
