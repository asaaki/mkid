+++
key = "uuid-v7-default"
tags = [
    "architecture",
    "performance",
]
created = "2026-01-30T12:29:13.111754Z"
updated = "2026-01-30T12:29:13.111754Z"
+++

UUID v7 chosen as default for mkid because it provides database performance (time-ordered UUIDs reduce B-tree fragmentation up to 10x vs v4), sortability (lexicographic comparison works on raw bytes), modern Unix timestamps in milliseconds, and monotonicity guarantees. Research from RFC 9562 shows v1/v4 cause negative database performance effects.
