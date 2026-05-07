# Brahma 🕉️

Brahma is a lightweight and powerful project scaffolder written in Rust. It helps you quickly bootstrap new projects with sensible defaults or pre-defined templates.

## Features

- 🚀 **Fast Scaffolding**: Create a new project in seconds.
- 📦 **Template Support**: Bootstrap projects with specific stacks (e.g., Express.js).
- 🛠️ **Automatic Setup**: Initializes Git and installs dependencies automatically.
- 💻 **CLI First**: Simple and intuitive command-line interface.

## Installation

To build Brahma from source, ensure you have [Rust](https://www.rust-lang.org/) installed, then:

```bash
git clone https://github.com/yourusername/brahma.git
cd brahma
cargo build --release
```

The binary will be available at `target/release/brahma`.

## Usage

### Create a Basic Project

To create a basic project with a `.gitignore`, `README.md`, and an initialized Git repository:

```bash
brahma new my-cool-project
```

### Create a Project with a Template

To create a project using a template (e.g., Express.js):

```bash
brahma new my-web-app --template
```

Or using the short flag:

```bash
brahma new my-web-app -t
```

Brahma will prompt you to select a template from the available options.

#### Example Output:

```text
$ brahma new my-web-app -t
? Select the project template ›
❯ Express

✔ Select the project template · express
[Brahma] Applying template: express...
[Brahma] Installing dependencies (express, nodemon)...
[Brahma] Running 'npm run dev'...

> my-web-app@1.0.0 dev
> nodemon src/index.js

[nodemon] 3.1.0
[nodemon] starting `node src/index.js`
Server is running on port 3000
Project my-web-app created
```

## Available Templates

| Template | Description | Included Features |
| :--- | :--- | :--- |
| **None** | Basic project structure | Git init, .gitignore, README.md |
| **Express** | Node.js Express server | package.json, src/index.js, npm install, dev script |

## Project Structure

Brahma organizes its source code as follows:

- `src/main.rs`: Entry point and CLI parsing.
- `src/cli.rs`: CLI command definitions using `clap`.
- `src/project_brahma/`: Core logic for project generation.
  - `project.rs`: Main project creation workflow.
  - `template_router.rs`: Routes requests to specific template generators.
  - `template_selector.rs`: Interactive template selection.
  - `template_brahma/`: Individual template implementations.
- `templates/`: Static assets for templates (embedded in the binary).

