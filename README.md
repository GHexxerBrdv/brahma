<p align="center">
  <img src="assets/brahma2.png" alt="Brahma Logo" width="200" />
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

```bash
curl -L https://raw.githubusercontent.com/GHexxerBrdv/brahma/main/install.sh | bash
```

> Note: as per the initial version of tool you have to manually run the above command to support new version everytime. we are pllanning to publish a package soon such that user can easily update to the latest version.

## Usage

### Create a Basic Project

To create a basic project with a `.gitignore`, `README.md`, and an initialized Git repository:

```bash
brahma create project-name
```

### Create a Project with a Template

To create a project using a template (e.g., Express.js):

```bash
brahma create project-name --template
```

Or using the short flag:

```bash
brahma create project-name -t
```

Brahma will prompt you to select a project from the available options.

## Available Templates

| Template | Description | Included Features |
| :--- | :--- | :--- |
| **Empty** | Basic project structure | Git init, .gitignore, README.md |
| **Express (JS)** | Node.js Express server (JavaScript) (no-git option is available) | MVC structure, package.json, src/app.js, dev script |
| **Express (TS)** | Node.js Express server (TypeScript) (no-git option is available) | MVC structure, tsconfig.json, src/app.ts, dev script |
