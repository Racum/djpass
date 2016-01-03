use std::process::Command;


#[test]
fn test_single_no_arguments() {
    let command = Command::new("target/debug/djpass").output().unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert!(output.starts_with("Password: "));
}

#[test]
fn test_single_password_argument() {
    let command = Command::new("target/debug/djpass").arg("hello").output().unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert!(output.starts_with("\u{1b}[32mHash:[0m pbkdf2_sha256$"));
}

#[test]
fn test_password_and_algorithm_arguments() {
    let command = Command::new("target/debug/djpass")
                      .arg("hello")
                      .arg("-a")
                      .arg("sha1")
                      .output()
                      .unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert!(output.starts_with("\u{1b}[32mHash:[0m sha1$"));
}

#[test]
fn test_good_password_and_hash_arguments() {
    let command = Command::new("target/debug/djpass")
                      .arg("hello")
                      .arg("sha1$hzPiRIKYykm8$231fe9a64fe025a2b0c89efb132d518502c6fac9")
                      .output()
                      .unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert!(output.starts_with("\u{1b}[32mPassword ok."));
}

#[test]
fn test_bad_password_and_hash_arguments() {
    let command = Command::new("target/debug/djpass")
                      .arg("helloz")
                      .arg("sha1$hzPiRIKYykm8$231fe9a64fe025a2b0c89efb132d518502c6fac9")
                      .output()
                      .unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert!(output.starts_with("\u{1b}[31mPassword does not match hash."));
}

#[test]
fn test_ignore_algoritm_when_verifying() {
    let command = Command::new("target/debug/djpass")
                      .arg("hello")
                      .arg("-a")
                      .arg("sha1")
                      .arg("sha1$hzPiRIKYykm8$231fe9a64fe025a2b0c89efb132d518502c6fac9")
                      .output()
                      .unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    println!("{}", output);
    assert!(output.starts_with("\u{1b}[33mAlgorithm ignored for verification."));
}

#[test]
fn test_bad_algorithm() {
    let command = Command::new("target/debug/djpass")
                      .arg("hello")
                      .arg("-a")
                      .arg("bad")
                      .output()
                      .unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    println!("{}", output);
    assert!(output.starts_with("\u{1b}[31mAlgorithm not supported."));
}

#[test]
fn test_bad_hash() {
    let command = Command::new("target/debug/djpass").arg("hello").arg("blah").output().unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert!(output.starts_with("\u{1b}[31mHash is not properly formatted."));
}

#[test]
fn test_version() {
    let command = Command::new("target/debug/djpass").arg("hello").arg("-v").output().unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert!(output.starts_with("djpass 0.1.0, generates hashes for Django 1.9"));
}

#[test]
fn test_help() {
    let command = Command::new("target/debug/djpass").arg("hello").arg("-h").output().unwrap();
    let output = String::from_utf8_lossy(&command.stdout);
    assert!(output.starts_with("Generates or validates password hashes used in Django Project."));
}
