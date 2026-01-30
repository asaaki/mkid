+++
key = "copypasta-error-handling"
tags = [
    "rust",
    "error-handling",
]
created = "2026-01-30T12:29:18.473030Z"
updated = "2026-01-30T12:29:18.473030Z"
+++

The copypasta crate returns Box<dyn Error> which is not directly compatible with anyhow::Context. Solution: use .map_err(|e| anyhow::anyhow!("message: {}", e)) instead of .context(). This provides proper error conversion and helpful error messages for clipboard failures on headless systems.
