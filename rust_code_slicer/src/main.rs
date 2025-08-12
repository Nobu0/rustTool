use clap::Parser;
use std::fs;
use syn::{File, Item};

/// A simple tool to slice Rust code.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The input file to use
    #[arg(long)]
    input: String,

    /// The output file to write to
    #[arg(long)]
    output: String,

    /// The name of the item to extract
    #[arg(long)]
    item_name: String,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let source_code = fs::read_to_string(&cli.input)?;

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
            if ident.to_string() == cli.item_name {
                found_item = Some(item);
                break;
            }
        }
    }

    if let Some(item) = found_item {
        let file_to_print = syn::File {
            shebang: None,
            attrs: vec![],
            items: vec![item],
        };

        let formatted_code = prettyplease::unparse(&file_to_print);
        fs::write(&cli.output, formatted_code)?;
        println!("Successfully extracted item '{}' to '{}'", cli.item_name, cli.output);
    } else {
        eprintln!("Item '{}' not found in '{}'", cli.item_name, cli.input);
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Specified item not found in source file",
        ));
    }

    Ok(())
}
