# Brahma Architecture

Brahma follows a core-plus-extension design.

- apps/cli: user-facing command-line binary
- packages/core: orchestration engine, validation, rollback, template rendering
- extensions/*: framework-specific generators and templates
- templates/shared: cross-framework reusable assets
