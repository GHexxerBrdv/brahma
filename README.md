<p align="center">
  <img src="assets/brahma.png" alt="Brahma Logo" width="200" />
</p>

<h1 align="center">Brahma 🕉️</h1>

<p align="center">
  <strong>A lightweight and powerful project scaffolder for the modern developer.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge" alt="License" />
</p>

---

Brahma is designed to help you quickly bootstrap new projects with sensible defaults or pre-defined templates. No more wasting time on repetitive boilerplate setup.

## Features

- **Fast Scaffolding**: Create a new project in seconds.
- **Template Support**: Bootstrap projects with specific stacks (e.g., Express.js).
- **Automatic Setup**: Initializes Git and installs dependencies automatically.
- **CLI First**: Simple and intuitive command-line interface.
- **Robust Logic**: Handles directory creation and template rendering flawlessly.

## Installation

To build Brahma from source, ensure you have [Rust](https://www.rust-lang.org/) installed, then:

```bash
git clone https://github.com/GHexxerBrdv/brahma.git
cd brahma
cargo build --release
```

The binary will be available at `target/release/brahma`.

Or you can install it globally using `cargo install`:

```bash
cargo install --git https://github.com/GHexxerBrdv/brahma.git
```

> Note: as per the initial version of tool everytime you have to manually run the above command to support new version.

## Usage

### Create a Basic Project

To create a basic project with a `.gitignore`, `README.md`, and an initialized Git repository:

```bash
brahma create my-cool-project
```

### Create a Project with a Template

To create a project using a template (e.g., Express.js):

```bash
brahma create my-web-app --template
```

Or using the short flag:

```bash
brahma create my-web-app -t
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

Brahma is built with modularity in mind:

```text
src/
├── main.rs                 # Entry point and CLI parsing
├── cli.rs                  # CLI command definitions
└── project_brahma/         # Core logic
    ├── project.rs          # Workflow management
    ├── template_router.rs  # Template routing
    ├── template_selector.rs # Interactive UI
    └── template_brahma/    # Implementations
```

