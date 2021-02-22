use std::{process::Command, str};

fn main() {
    let git_branch = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .unwrap();
    let git_branch = str::from_utf8(git_branch.stdout.as_slice()).unwrap();

    let git_commit = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    let git_commit = str::from_utf8(git_commit.stdout.as_slice()).unwrap();

    println!("cargo:rustc-env=GIT_BRANCH={}", git_branch);
    println!("cargo:rustc-env=GIT_COMMIT={}", git_commit);
}
