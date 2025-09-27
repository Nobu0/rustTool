use clap::{Parser, ValueEnum};
use std::fs;
use std::io::{self, Read};
use syn::spanned::Spanned;
use syn::{File, Item};

// Githubの更新テスト3回目
/// Defines the possible output formats for the tool.
#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    /// Output the extracted Rust code, formatted by `prettyplease`.
    Code,
    /// Output the debug representation of the Abstract Syntax Tree (AST) for the item.
    Ast,
    /// Output a CSV summary of all items in the source file.
    Csv,
}

/// A simple tool to parse Rust source code and extract specific items or metadata.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The path to the Rust source file to process.
    /// If not provided, the tool will read from standard input (stdin).
    #[arg(long)]
    input: Option<String>,

    /// The path to the output file.
    /// If not provided, the tool will write to standard output (stdout).
    #[arg(long)]
    output: Option<String>,

    /// The name of a specific top-level item (e.g., function, struct) to extract.
    /// If not provided, the entire file is processed.
    #[arg(long)]
    item_name: Option<String>,

    /// The desired output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Ast)]
    format: OutputFormat,
}

fn main() -> std::io::Result<()> {
    // Parse command-line arguments.
    let cli = Cli::parse();

    // Read the source code from either a file or stdin.
    let source_code = if let Some(ref input_path) = cli.input {
        fs::read_to_string(input_path)?
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf
    };

    // Parse the source code into an Abstract Syntax Tree (AST).
    let ast: File = match syn::parse_file(&source_code) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error parsing source file: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to parse Rust code",
            ));
        }
    };

    // --- Special Handling for CSV Format ---
    // If the requested format is CSV, we generate the CSV for the whole file and exit early.
    // This mode ignores the `item_name` argument as it provides a summary of all items.
    if matches!(cli.format, OutputFormat::Csv) {
        let csv_data = generate_csv(&ast).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        if let Some(output_path) = cli.output {
            fs::write(&output_path, &csv_data)?;
        } else {
            print!("{}", csv_data);
        }
        return Ok(());
    }

    // --- Main Logic for Code/AST Slicing ---
    // This tuple will hold the final string to be printed and the name of the item if one was found.
    let (output_string, found_item_name) = if let Some(item_name) = &cli.item_name {
        // Case 1: A specific item_name was provided. Find it in the AST.
        let mut found_item: Option<Item> = None;
        for item in ast.items {
            let item_ident = match &item {
                Item::Fn(i) => Some(&i.sig.ident),
                Item::Struct(i) => Some(&i.ident),
                Item::Enum(i) => Some(&i.ident),
                Item::Mod(i) => Some(&i.ident),
                Item::Trait(i) => Some(&i.ident),
                Item::Macro(i) => i.ident.as_ref(),
                _ => None,
            };

            if let Some(ident) = item_ident {
                if ident.to_string() == *item_name {
                    found_item = Some(item);
                    break;
                }
            }
        }

        if let Some(item) = found_item {
            // The item was found, generate the output string based on the chosen format.
            let output = match cli.format {
                OutputFormat::Code => {
                    // To print just one item, we create a new temporary `File` AST.
                    let file_to_print = syn::File {
                        shebang: None,
                        attrs: vec![],
                        items: vec![item],
                    };
                    prettyplease::unparse(&file_to_print)
                }
                OutputFormat::Ast => format!("{:#?}", item),
                OutputFormat::Csv => unreachable!(), // Handled above.
            };
            (output, Some(item_name.clone()))
        } else {
            // The specified item was not found in the file.
            let input_source = cli.input.as_deref().unwrap_or("stdin");
            eprintln!("Item '{}' not found in '{}'", item_name, input_source);
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Specified item not found in source file",
            ));
        }
    } else {
        // Case 2: No item_name was provided. Process the whole file.
        let output = match cli.format {
            OutputFormat::Code => prettyplease::unparse(&ast),
            OutputFormat::Ast => format!("{:#?}", ast),
            OutputFormat::Csv => unreachable!(), // Handled above.
        };
        (output, None)
    };

    // --- Final Output ---
    // Write the generated string to the specified output (file or stdout).
    if let Some(output_path) = cli.output {
        fs::write(&output_path, &output_string)?;
        // Print a success message only when a specific item is extracted as code to a file.
        if let (Some(item_name), OutputFormat::Code) = (found_item_name, &cli.format) {
             println!("Successfully extracted item '{}' to '{}'", item_name, output_path);
        }
    } else {
        print!("{}", output_string);
    }

    Ok(())
}

/// Helper function to get all attributes of an item as a single formatted string.
fn get_attrs_string(attrs: &[syn::Attribute]) -> String {
    attrs
        .iter()
        .map(|attr| quote::quote!(#attr).to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generates a CSV summary of all top-level items in a Rust source file.
fn generate_csv(file: &syn::File) -> Result<String, csv::Error> {
    // Create a CSV writer that writes to an in-memory buffer (a Vec<u8>).
    let mut wtr = csv::Writer::from_writer(vec![]);
    // Write the header row.
    wtr.write_record(&["item_name", "item_type", "start_line", "end_line", "attributes"])?;

    // Iterate over each top-level item in the AST.
    for item in &file.items {
        // Match on the item type to extract its details.
        let (item_name, item_type, start_line, end_line, attributes) = match item {
            Item::Fn(i) => (i.sig.ident.to_string(), "Function", i.span().start().line, i.span().end().line, get_attrs_string(&i.attrs)),
            Item::Struct(i) => (i.ident.to_string(), "Struct", i.span().start().line, i.span().end().line, get_attrs_string(&i.attrs)),
            Item::Enum(i) => (i.ident.to_string(), "Enum", i.span().start().line, i.span().end().line, get_attrs_string(&i.attrs)),
            Item::Mod(i) => (i.ident.to_string(), "Module", i.span().start().line, i.span().end().line, get_attrs_string(&i.attrs)),
            Item::Trait(i) => (i.ident.to_string(), "Trait", i.span().start().line, i.span().end().line, get_attrs_string(&i.attrs)),
            Item::Macro(i) => (i.ident.as_ref().map_or_else(String::new, |id| id.to_string()), "Macro", i.span().start().line, i.span().end().line, get_attrs_string(&i.attrs)),
            Item::Impl(i) => {
                // `impl` blocks don't have a simple name, so we construct one.
                // e.g., "impl MyTrait for MyStruct"
                let file_with_impl = syn::File { shebang: None, attrs: vec![], items: vec![item.clone()] };
                let name = prettyplease::unparse(&file_with_impl);
                // We just want the 'impl Trait for Type' part, not the whole block.
                let name = name.lines().next().unwrap_or("").trim().trim_end_matches('{').trim().to_string();
                (name, "Impl", i.span().start().line, i.span().end().line, get_attrs_string(&i.attrs))
            }
            Item::Use(i) => {
                // For `use` statements, we format the whole statement as its name.
                let file_with_use = syn::File { shebang: None, attrs: vec![], items: vec![item.clone()] };
                let name = prettyplease::unparse(&file_with_use).trim().trim_end_matches(';').to_string();
                (name, "Use", i.span().start().line, i.span().end().line, get_attrs_string(&i.attrs))
            }
            _ => continue, // Ignore other item types for now.
        };
        // Write the extracted data as a new row in the CSV.
        wtr.write_record(&[item_name, item_type.to_string(), start_line.to_string(), end_line.to_string(), attributes])?;
    }

    // Finalize the CSV writing process.
    wtr.flush()?;
    // Convert the in-memory buffer (Vec<u8>) into a String.
    let data = String::from_utf8(wtr.into_inner().expect("CSV writer into_inner failed"))
        .expect("CSV data is not valid UTF-8");
    Ok(data)
}
