+++
key = "clap-positional-version"
tags = [
    "cli-design",
    "ux",
]
created = "2026-01-30T12:29:24.780546Z"
updated = "2026-01-30T12:29:24.780546Z"
+++

Using positional argument for UUID version (mkid uuid v4) instead of flags (mkid uuid --v4) improves UX because only one version can be generated at a time. Clap configuration: version: Option<String> with value_parser = ["v1", "v3", ..., "v8"]. Makes CLI more intuitive and prevents user confusion.
