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

#[test]
fn test_extract_to_stdout() {
    let mut cmd = Command::cargo_bin("rust_code_slicer").unwrap();
    cmd.arg("--input=tests/complex_sample.rs")
        .arg("--item-name=Message");

    let expected = r#"
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
"#;

    let mut expected_with_newline = expected.trim().to_string();
    expected_with_newline.push('\n');

    cmd.assert()
        .success()
        .stdout(expected_with_newline);
}

#[test]
fn test_extract_ast() {
    let mut cmd = Command::cargo_bin("rust_code_slicer").unwrap();
    cmd.arg("--input=tests/complex_sample.rs")
        .arg("--item-name=ComplexStruct")
        .arg("--format=ast");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Item::Struct"))
        .stdout(predicate::str::contains("ident: Ident"))
        .stdout(predicate::str::contains("struct_token: Struct"));
}

#[test]
fn test_whole_file_as_code() {
    let mut cmd = Command::cargo_bin("rust_code_slicer").unwrap();
    cmd.arg("--input=tests/complex_sample.rs");

    let original_file_content = fs::read_to_string("tests/complex_sample.rs").unwrap();
    let expected_output = prettyplease::unparse(&syn::parse_file(&original_file_content).unwrap());

    cmd.assert().success().stdout(expected_output);
}

#[test]
fn test_whole_file_as_ast() {
    let mut cmd = Command::cargo_bin("rust_code_slicer").unwrap();
    cmd.arg("--input=tests/complex_sample.rs")
        .arg("--format=ast");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("File {"))
        .stdout(predicate::str::contains("shebang: None"))
        .stdout(predicate::str::contains("items: ["));
}
