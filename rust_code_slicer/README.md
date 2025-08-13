# Rust Code Slicer

A simple tool to parse Rust source code and extract specific items or metadata.

This tool can be used to "slice" a Rust source file to extract a single item (like a function or struct), or to analyze the structure of the code by outputting its Abstract Syntax Tree (AST) or a CSV summary.

## Usage

```
A simple tool to parse Rust source code and extract specific items or metadata

Usage: rust_code_slicer [OPTIONS]

Options:
      --input <INPUT>
          The path to the Rust source file to process. If not provided, the tool will read from standard input (stdin)

      --output <OUTPUT>
          The path to the output file. If not provided, the tool will write to standard output (stdout)

      --item-name <ITEM_NAME>
          The name of a specific top-level item (e.g., function, struct) to extract. If not provided, the entire file is processed

      --format <FORMAT>
          The desired output format

          Possible values:
          - code: Output the extracted Rust code, formatted by `prettyplease`
          - ast:  Output the debug representation of the Abstract Syntax Tree (AST) for the item
          - csv:  Output a CSV summary of all items in the source file

          [default: ast]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Examples

### 1. Extract a specific function as code

This command extracts the `standalone_function` from `tests/complex_sample.rs` and prints it as formatted code to standard output.

```sh
cargo run -- --input tests/complex_sample.rs --item-name standalone_function --format code
```

### 2. Get the AST of an item

This command gets the Abstract Syntax Tree for the `ComplexStruct` item and prints it to the file `ast_output.txt`. This is the default format.

```sh
cargo run -- --input tests/complex_sample.rs --item-name ComplexStruct --output ast_output.txt
```

### 3. Get a CSV summary of a file

This command analyzes all top-level items in `tests/complex_sample.rs` and outputs a CSV summary to `summary.csv`.

```sh
cargo run -- --input tests/complex_sample.rs --format csv --output summary.csv
```

### 4. Reformat a file using stdin/stdout

This command reads Rust code from standard input, reformats the entire file, and prints the result to standard output.

```sh
cat tests/complex_sample.rs | cargo run -- --format code
```
