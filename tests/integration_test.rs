use std::fs::{self, File};
use std::process::Command;

#[test]
fn test_zero_length_file() {
    let test_dir = "test_zero_length_dir";
    let _ = fs::remove_dir_all(test_dir);
    fs::create_dir_all(test_dir).unwrap();

    // Create a zero-length xml file
    File::create(format!("{}/empty.xml", test_dir)).unwrap();

    let output = Command::new("cargo")
        .args(&["run", "--manifest-path", concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml")])
        .current_dir(test_dir)
        .output()
        .expect("Failed to execute command");

    let _ = fs::remove_dir_all(test_dir);

    assert!(
        output.status.success(),
        "Command failed, possible panic on zero-length file"
    );
}
