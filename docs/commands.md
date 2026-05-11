# Commands

## Current Command Set

### `brahma create <name>`

Creates a minimal project scaffold.

What it currently does:

- creates project files from the `empty` template
- initializes a Git repository

Example:

```bash
brahma create my-service
```

### `brahma create <name> --template` or `-t`

Creates a project using an interactive template selector.

Current template options:

- `Empty`
- `Express (JS)`
- `Express (TS)`

Example:

```bash
brahma create my-api --template
```

## Development Commands

Run from repository root:

```bash
cargo run -p brahma -- create demo-service --template
```

Build/check:

```bash
cargo fmt --all
cargo check --workspace
cargo test --workspace
```

## Exit Behavior

- returns success (`0`) on completion
- returns error (`non-zero`) when generation or dependency steps fail
- surfaces contextual failure messages from core

## Planned Commands (Not Yet Implemented)

- `brahma add`: add capabilities to existing project
- `brahma generate`: scaffold resources/modules after project creation
- `brahma doctor`: detect template drift and project health issues
