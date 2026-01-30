+++
key = "module-structure"
tags = [
    "architecture",
    "maintainability",
]
created = "2026-01-30T12:29:30.147448Z"
updated = "2026-01-30T12:29:30.147448Z"
+++

Project uses clean 4-module architecture: cli.rs (Clap definitions), generator.rs (UUID generation logic), output.rs (formatting & clipboard), main.rs (orchestration). This separation makes code maintainable and testable. Future ID types (ULID, NanoID) can follow same pattern as additional modules.
