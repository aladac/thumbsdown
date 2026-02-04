use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn no_args_shows_error() {
    cargo_bin_cmd!("thumbsdown")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn help_flag() {
    cargo_bin_cmd!("thumbsdown")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("thumbnail"))
        .stdout(predicate::str::contains("--thumbs"))
        .stdout(predicate::str::contains("--columns"));
}

#[test]
fn version_flag() {
    cargo_bin_cmd!("thumbsdown")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("thumbsdown"));
}

#[test]
fn nonexistent_video_file() {
    cargo_bin_cmd!("thumbsdown")
        .arg("nonexistent_video_xyz.mp4")
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
}

#[test]
fn output_already_exists_without_force() {
    let dir = tempfile::tempdir().expect("tempdir");
    let video = dir.path().join("fake.mp4");
    std::fs::write(&video, b"not a real video").expect("write");
    let output = dir.path().join("existing.png");
    std::fs::write(&output, b"existing").expect("write");

    cargo_bin_cmd!("thumbsdown")
        .arg(&video)
        .arg("-o")
        .arg(&output)
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}
