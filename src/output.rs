use anyhow::Result;
use copypasta::{ClipboardContext, ClipboardProvider};
use uuid::Uuid;

/// UUID output format types
#[derive(Debug, Clone, Copy)]
pub enum FormatType {
    Hyphenated,
    Simple,
    Urn,
    Braced,
}

impl FormatType {
    /// Parse format string to FormatType
    pub fn from_str(s: &str) -> Self {
        match s {
            "simple" => FormatType::Simple,
            "urn" => FormatType::Urn,
            "braced" => FormatType::Braced,
            _ => FormatType::Hyphenated,
        }
    }
}

/// Format a UUID according to the specified format and case
pub fn format_uuid(uuid: &Uuid, uppercase: bool, format: FormatType) -> String {
    let mut buffer = Uuid::encode_buffer();

    let formatted = match format {
        FormatType::Hyphenated => {
            if uppercase {
                uuid.hyphenated().encode_upper(&mut buffer)
            } else {
                uuid.hyphenated().encode_lower(&mut buffer)
            }
        }
        FormatType::Simple => {
            if uppercase {
                uuid.simple().encode_upper(&mut buffer)
            } else {
                uuid.simple().encode_lower(&mut buffer)
            }
        }
        FormatType::Urn => {
            if uppercase {
                uuid.urn().encode_upper(&mut buffer)
            } else {
                uuid.urn().encode_lower(&mut buffer)
            }
        }
        FormatType::Braced => {
            if uppercase {
                uuid.braced().encode_upper(&mut buffer)
            } else {
                uuid.braced().encode_lower(&mut buffer)
            }
        }
    };

    formatted.to_string()
}

/// Copy text to clipboard
pub fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut ctx = ClipboardContext::new().map_err(|e| {
        anyhow::anyhow!(
            "Failed to initialize clipboard: {}. Is a display server running?",
            e
        )
    })?;

    ctx.set_contents(text.to_owned())
        .map_err(|e| anyhow::anyhow!("Failed to copy to clipboard: {}", e))?;

    Ok(())
}
