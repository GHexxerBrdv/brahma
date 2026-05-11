# Brahma Architecture

## Overview

Brahma is a Rust workspace that follows a **core + extension** architecture:

- `apps/cli`: user-facing binary crate (`brahma`)
- `packages/core`: orchestration engine and generation logic
- `extensions/*`: framework-specific metadata and templates
- `templates/shared`: shared templates used across extensions

This design keeps the CLI stable while allowing framework support to grow independently.

## Workspace Layout

```txt
brahma/
├── apps/cli
├── packages/core
├── extensions/
│   ├── express/
│   └── nest/
└── templates/shared
```

## Responsibilities

### `apps/cli`

- Parses command-line input with `clap`
- Hands execution off to `packages/core`
- Contains no framework-specific generation logic

### `packages/core`

- Defines CLI command contracts (`create`)
- Selects templates
- Routes to framework generators
- Renders files from embedded template directories
- Executes dependency install commands
- Initializes Git in generated projects
- Surfaces contextual errors

### `extensions/*`

- Own framework-specific template assets
- Ship metadata in `extension.yaml`
- Keep framework details isolated from core

### `templates/shared`

- Stores reusable, cross-framework templates (for example empty starter assets)
- Intended to host future shared docker, CI, security, and health templates

## Generation Flow

```txt
User runs brahma create
  -> CLI parses args
  -> Core decides project type
  -> Router selects generator
  -> Template renderer resolves template files
  -> Files written to target project
  -> Dependencies installed (template-specific)
  -> Git initialized
```

## Template Resolution

The core renderer resolves template files in this order:

1. `templates/shared`
2. `extensions/*`

This allows shared defaults while keeping extension-specific templates authoritative.

## Error Handling

- `anyhow` and contextual `.context(...)` wrapping provide actionable failures.
- Command failures return typed errors from core and stop execution.
- Current implementation is safe and explicit; full transactional rollback remains a planned enhancement.

## Design Principles

- Keep the core small, stable, and framework-agnostic.
- Put framework logic in extensions, not the CLI.
- Prefer explicit contracts (metadata + directory structure).
- Optimize for maintainability and future polyglot support.
