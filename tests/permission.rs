use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

#[test]
fn test_permission_error_printed() {
    fs::create_dir("test_no_perm").unwrap();
    let mut perms = fs::metadata("test_no_perm").unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions("test_no_perm", perms.clone()).unwrap();

    let output = Command::new("cargo").args(&["run"]).output().unwrap();

    perms.set_mode(0o755);
    fs::set_permissions("test_no_perm", perms).unwrap();
    fs::remove_dir("test_no_perm").unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stderr.contains("test_no_perm") || stdout.contains("test_no_perm"),
        "Output should contain the permission error"
    );
}
