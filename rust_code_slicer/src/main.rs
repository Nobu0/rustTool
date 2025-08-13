use clap::{Parser, ValueEnum};
use std::fs;
use std::io::{self, Read};
use syn::spanned::Spanned;
use syn::{File, Item};

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    Code,
    Ast,
    Csv,
}

/// A simple tool to slice Rust code.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The input file to use. If not provided, reads from stdin.
    #[arg(long)]
    input: Option<String>,

    /// The output file to write to. If not provided, prints to stdout.
    #[arg(long)]
    output: Option<String>,

    /// The name of the item to extract. If not provided, the whole file is processed.
    #[arg(long)]
    item_name: Option<String>,

    /// The output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Ast)]
    format: OutputFormat,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let source_code = if let Some(ref input_path) = cli.input {
        fs::read_to_string(input_path)?
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf
    };

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

    // If format is CSV, we ignore everything else and just generate the CSV for the whole file.
    if matches!(cli.format, OutputFormat::Csv) {
        let csv_data = generate_csv(&ast).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        if let Some(output_path) = cli.output {
            fs::write(&output_path, &csv_data)?;
        } else {
            print!("{}", csv_data);
        }
        return Ok(());
    }

    let (output_string, found_item_name) = if let Some(item_name) = &cli.item_name {
        // Case 1: Find a specific item
        let mut found_item: Option<Item> = None;
        for item in ast.items {
            let item_ident = match &item {
                Item::Fn(item_fn) => Some(&item_fn.sig.ident),
                Item::Struct(item_struct) => Some(&item_struct.ident),
                Item::Enum(item_enum) => Some(&item_enum.ident),
                Item::Mod(item_mod) => Some(&item_mod.ident),
                Item::Trait(item_trait) => Some(&item_trait.ident),
                Item::Macro(item_macro) => item_macro.ident.as_ref(),
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
            let output = match cli.format {
                OutputFormat::Code => {
                    let file_to_print = syn::File {
                        shebang: None,
                        attrs: vec![],
                        items: vec![item],
                    };
                    prettyplease::unparse(&file_to_print)
                }
                OutputFormat::Ast => format!("{:#?}", item),
                OutputFormat::Csv => unreachable!(), // Handled above
            };
            (output, Some(item_name.clone()))
        } else {
            let input_source = cli.input.as_deref().unwrap_or("stdin");
            eprintln!("Item '{}' not found in '{}'", item_name, input_source);
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Specified item not found in source file",
            ));
        }
    } else {
        // Case 2: Process the whole file
        let output = match cli.format {
            OutputFormat::Code => prettyplease::unparse(&ast),
            OutputFormat::Ast => format!("{:#?}", ast),
            OutputFormat::Csv => unreachable!(), // Handled above
        };
        (output, None)
    };

    if let Some(output_path) = cli.output {
        fs::write(&output_path, &output_string)?;
        if let (Some(item_name), OutputFormat::Code) = (found_item_name, &cli.format) {
             println!("Successfully extracted item '{}' to '{}'", item_name, output_path);
        }
    } else {
        print!("{}", output_string);
    }

    Ok(())
}

fn generate_csv(file: &syn::File) -> Result<String, csv::Error> {
    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&["item_name", "item_type", "start_line", "end_line"])?;

    for item in &file.items {
        let (item_name, item_type, start_line, end_line) = match item {
            Item::Fn(item_fn) => (
                item_fn.sig.ident.to_string(),
                "Function",
                item.span().start().line,
                item.span().end().line,
            ),
            Item::Struct(item_struct) => (
                item_struct.ident.to_string(),
                "Struct",
                item.span().start().line,
                item.span().end().line,
            ),
            Item::Enum(item_enum) => (
                item_enum.ident.to_string(),
                "Enum",
                item.span().start().line,
                item.span().end().line,
            ),
            Item::Mod(item_mod) => (
                item_mod.ident.to_string(),
                "Module",
                item.span().start().line,
                item.span().end().line,
            ),
            Item::Trait(item_trait) => (
                item_trait.ident.to_string(),
                "Trait",
                item.span().start().line,
                item.span().end().line,
            ),
            Item::Macro(item_macro) => (
                item_macro.ident.as_ref().map_or_else(String::new, |i| i.to_string()),
                "Macro",
                item.span().start().line,
                item.span().end().line,
            ),
            Item::Impl(item_impl) => {
                let trait_name = item_impl.trait_.as_ref().map_or_else(String::new, |t| {
                    let path = &t.1;
                    quote::quote!(#path).to_string()
                });
                let type_name = match item_impl.self_ty.as_ref() {
                    syn::Type::Path(type_path) => quote::quote!(#type_path).to_string(),
                    _ => String::from(".."),
                };
                let name = if trait_name.is_empty() {
                    format!("impl {}", type_name)
                } else {
                    format!("impl {} for {}", trait_name, type_name)
                };
                (name, "Impl", item.span().start().line, item.span().end().line)
            }
            _ => continue, // Ignore other item types for now
        };
        wtr.write_record(&[item_name, item_type.to_string(), start_line.to_string(), end_line.to_string()])?;
    }

    wtr.flush()?;
    let data = String::from_utf8(wtr.into_inner().expect("CSV writer into_inner failed"))
        .expect("CSV data is not valid UTF-8");
    Ok(data)
}
