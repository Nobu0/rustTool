use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

// A helper function to reduce boilerplate
fn run_slicer(item_name: &str) -> (String, tempfile::TempDir) {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.rs");

    let mut cmd = Command::cargo_bin("rust_code_slicer").unwrap();
    cmd.arg("--input=tests/complex_sample.rs")
        .arg(format!("--output={}", output_path.display()))
        .arg(format!("--item-name={}", item_name));

    cmd.assert().success();

    let output_contents = fs::read_to_string(output_path).unwrap();
    (output_contents, temp_dir)
}

#[test]
fn test_extract_function() {
    let (output_contents, _temp_dir) = run_slicer("standalone_function");
    // NOTE: Inner comments are not preserved, which is an accepted limitation.
    let expected = r#"
#[allow(unused_variables)]
pub fn standalone_function(arg1: &str) {
    let x = 10;
    println!("Hello, {}!", arg1);
}
"#;
    assert_eq!(output_contents.trim(), expected.trim());
}

#[test]
fn test_extract_struct() {
    let (output_contents, _temp_dir) = run_slicer("ComplexStruct");
    let expected = r#"
/// A doc comment for a struct.
#[derive(Debug, Clone)]
pub struct ComplexStruct {
    pub name: String,
    pub value: i64,
}
"#;
    assert_eq!(output_contents.trim(), expected.trim());
}

#[test]
fn test_extract_module() {
    let (output_contents, _temp_dir) = run_slicer("inner_module");
    // NOTE: Inner comments are not preserved.
    let expected = r#"
pub mod inner_module {
    pub fn inner_function() {
        println!("I am inside a module.");
    }
    pub struct NestedStruct {
        pub data: Vec<u8>,
    }
}
"#;
    assert_eq!(output_contents.trim(), expected.trim());
}

#[test]
fn test_extract_macro() {
    let (output_contents, _temp_dir) = run_slicer("my_macro");
    let expected = r#"
#[macro_export]
macro_rules! my_macro {
    () => {
        println!("This is a macro!");
    };
}
"#;
    assert_eq!(output_contents.trim(), expected.trim());
}

#[test]
fn test_item_not_found() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.rs");

    let mut cmd = Command::cargo_bin("rust_code_slicer").unwrap();
    cmd.arg("--input=tests/complex_sample.rs")
        .arg(format!("--output={}", output_path.display()))
        .arg("--item-name=non_existent_item");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Item 'non_existent_item' not found"));
}
