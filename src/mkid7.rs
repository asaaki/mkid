// mkid7 - Quick UUID v7 generator
// Shortcut for: mkid uuid v7 (or just: mkid)
// Supports flags: --clipboard, --uppercase, --format, --count

mod cli;
mod generator;
mod output;

use anyhow::Result;
use clap::Parser;
use cli::UuidArgs;
use generator::generate_uuid;
use output::{FormatType, copy_to_clipboard, format_uuid};

#[derive(Parser)]
#[command(
    name = "mkid7",
    about = "Generate UUID v7 (sortable)",
    long_about = "Quick shortcut to generate UUID v7 (sortable, timestamp-based).\nEquivalent to: mkid uuid v7"
)]
struct Mkid7 {
    /// Copy output to clipboard
    #[arg(short, long)]
    clipboard: bool,

    #[command(flatten)]
    uuid_args: UuidArgs,
}

fn main() -> Result<()> {
    let mkid7 = Mkid7::parse();

    // Force version to v7
    let mut uuid_args = mkid7.uuid_args;
    uuid_args.version = Some("v7".to_string());

    // Determine format type
    let format_type = FormatType::from_str(&uuid_args.format);

    // Generate UUIDs based on count
    let mut outputs = Vec::new();
    for _ in 0..uuid_args.count {
        let uuid = generate_uuid(&uuid_args)?;
        let formatted = format_uuid(&uuid, uuid_args.uppercase, format_type);
        outputs.push(formatted);
    }

    // Print to stdout
    for output in &outputs {
        println!("{}", output);
    }

    // Copy to clipboard if requested
    if mkid7.clipboard {
        let clipboard_content = outputs.join("\n");
        copy_to_clipboard(&clipboard_content)?;
    }

    Ok(())
}
