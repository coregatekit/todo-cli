# Todo CLI

A simple command-line to-do list application written in Rust.

## Features

- Add tasks with a title
- List all tasks with their status
- Mark tasks as done
- Remove individual tasks by ID
- Clear all tasks at once
- Persists tasks to a JSON file (`~/.todo_cli/todo.json` by default)

## Installation

```sh
cargo install --path .
```

## Usage

```
todo [OPTIONS] <COMMAND>
```

### Commands

| Command | Description | Example |
|---------|-------------|---------|
| `add <title>` | Add a new todo item | `todo add "learn rust"` |
| `list` | List all todo items | `todo list` |
| `done <id>` | Mark an item as done by ID | `todo done 1` |
| `rm <id>` | Remove an item by ID | `todo rm 2` |
| `clean` | Clear all items | `todo clean` |

### Options

| Option | Description |
|--------|-------------|
| `--store <PATH>` | Use a custom JSON file path instead of the default |

### Example session

```sh
$ todo add "learn rust"
Added: learn rust

$ todo add "write tests"
Added: write tests

$ todo list
1. [ ] learn rust
2. [ ] write tests

$ todo done 1
Done: 1

$ todo list
1. [x] learn rust
2. [ ] write tests

$ todo rm 2
Removed: 2

$ todo clean
Cleared 1 items
```

## Data Storage

Tasks are stored as JSON at `~/.todo_cli/todo.json` by default. Use `--store` to override:

```sh
todo --store ./my-tasks.json add "custom store"
```

## Development

### Run tests

```sh
cargo test
```

### Build

```sh
cargo build --release
```

## Dependencies

- [clap](https://github.com/clap-rs/clap) — CLI argument parsing
- [serde](https://serde.rs/) / [serde_json](https://github.com/serde-rs/json) — JSON serialization


