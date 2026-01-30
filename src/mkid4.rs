// mkid4 - Quick UUID v4 generator
// Shortcut for: mkid uuid v4
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
    name = "mkid4",
    about = "Generate UUID v4 (random)",
    long_about = "Quick shortcut to generate UUID v4 (random).\nEquivalent to: mkid uuid v4"
)]
struct Mkid4 {
    /// Copy output to clipboard
    #[arg(short, long)]
    clipboard: bool,

    #[command(flatten)]
    uuid_args: UuidArgs,
}

fn main() -> Result<()> {
    let mkid4 = Mkid4::parse();

    // Force version to v4
    let mut uuid_args = mkid4.uuid_args;
    uuid_args.version = Some("v4".to_string());

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
    if mkid4.clipboard {
        let clipboard_content = outputs.join("\n");
        copy_to_clipboard(&clipboard_content)?;
    }

    Ok(())
}
