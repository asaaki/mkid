use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "mkid",
    about = "Generate unique IDs",
    long_about = "Generate unique IDs\n\nWhen called without arguments, generates a UUID v7 (lowercase, hyphenated).\nUse 'mkid uuid' subcommand for more options."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Copy output to clipboard
    #[arg(short, long, global = true)]
    pub clipboard: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate UUIDs
    Uuid(UuidArgs),
}

#[derive(Args)]
pub struct UuidArgs {
    /// UUID version (v1-v8), defaults to v7
    #[arg(value_parser = ["v1", "v3", "v4", "v5", "v6", "v7", "v8"])]
    pub version: Option<String>,

    /// Output format
    #[arg(long, value_parser = ["hyphenated", "simple", "urn", "braced"], default_value = "hyphenated")]
    pub format: String,

    /// Output in uppercase
    #[arg(short, long)]
    pub uppercase: bool,

    /// Number of UUIDs to generate
    #[arg(long, default_value = "1")]
    pub count: usize,

    // Version-specific parameters for v1/v6
    /// Node ID as 12 hex characters (for v1/v6)
    #[arg(long)]
    pub node_id: Option<String>,

    /// Use random node ID (for v1/v6)
    #[arg(long)]
    pub random_node: bool,

    // Version-specific parameters for v3/v5
    /// Namespace: dns, url, oid, x500, or custom UUID (for v3/v5)
    #[arg(long)]
    pub namespace: Option<String>,

    /// Name to hash (for v3/v5)
    #[arg(long)]
    pub name: Option<String>,

    // Version-specific parameters for v8
    /// Custom bytes as 32 hex characters (for v8)
    #[arg(long)]
    pub bytes: Option<String>,
}

impl UuidArgs {
    /// Create default v7 configuration
    pub fn default_v7() -> Self {
        Self {
            version: Some("v7".to_string()),
            format: "hyphenated".to_string(),
            uppercase: false,
            count: 1,
            node_id: None,
            random_node: false,
            namespace: None,
            name: None,
            bytes: None,
        }
    }
}
