# Extension API

## Purpose

The extension API defines how framework-specific scaffolders plug into Brahma without expanding core complexity.

## Extension Package Contract

Each extension directory should include:

```txt
extensions/<name>/
├── extension.yaml
├── templates/
├── src/        # optional hook code (future)
└── tests/
```

## `extension.yaml` Contract

Required fields:

- `id`: unique extension identifier (`@brahma/<name>`)
- `name`: human-readable name
- `apiVersion`: extension contract version (for compatibility checks)
- `engineVersionRange`: supported Brahma core versions
- `capabilities`: list of supported actions

Current examples:

- `extensions/express/extension.yaml`
- `extensions/nest/extension.yaml`

## Capabilities

Capability values represent which lifecycle commands the extension supports.
Current known capability values:

- `init`
- `add` (planned)
- `generate` (planned)
- `doctor` (planned)

Core should only call features explicitly declared by the extension.

## Template Ownership Rule

- framework-specific files must live in `extensions/<name>/templates`
- shared reusable assets belong in `templates/shared`
- avoid duplicating shared content across extensions

## Versioning Rules

- bump `apiVersion` when extension contract shape changes
- update `engineVersionRange` when extension requires new core behavior
- keep backward compatibility where possible to avoid breaking generated workflows

## Validation Checklist

Before marking an extension as stable:

- metadata schema is valid
- declared capabilities match implemented behavior
- template generation succeeds end-to-end
- generated project builds/runs
- docs explain available options and defaults
