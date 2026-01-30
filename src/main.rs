mod cli;
mod generator;
mod output;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use generator::generate_uuid;
use output::{FormatType, copy_to_clipboard, format_uuid};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Default to UUID v7 if no subcommand
    let uuid_args = match cli.command {
        Some(Commands::Uuid(args)) => args,
        None => cli::UuidArgs::default_v7(),
    };

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
    if cli.clipboard {
        let clipboard_content = outputs.join("\n");
        copy_to_clipboard(&clipboard_content)?;
    }

    Ok(())
}
