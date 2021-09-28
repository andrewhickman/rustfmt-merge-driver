use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use std::process::Command;

const BASE_FILE: &str = r#"fn main() { println!("Hello, world!") }
"#;
const CURRENT_FILE: &str = r#"fn main() {
    println!("Hello, world!")
}
"#;
const OTHER_FILE: &str = r#"fn main() { println!("Goodbye, world!") }
"#;

const MERGED_FILE: &str = r#"fn main() {
    println!("Goodbye, world!")
}
"#;

#[test]
fn merge() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = assert_fs::TempDir::new()?;

    tempdir.child("base.rs").write_str(BASE_FILE)?;
    tempdir.child("current.rs").write_str(CURRENT_FILE)?;
    tempdir.child("other.rs").write_str(OTHER_FILE)?;

    Command::cargo_bin("rustfmt-merge-driver")?
        .current_dir(tempdir.path())
        .arg("current.rs")
        .arg("base.rs")
        .arg("other.rs")
        .assert()
        .success();

    tempdir.child("current.rs").assert(MERGED_FILE);
    Ok(())
}
