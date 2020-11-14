use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::io::Write;
use std::process::Command; // Run programs
use tempfile::NamedTempFile;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("zeroc")?;

    cmd.arg("-p nofile");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: Os { code: 2, kind: NotFound",
    ));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("zeroc")?;
    cmd.arg("-p").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("204"));

    Ok(())
}
