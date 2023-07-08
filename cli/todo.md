# Zipity CLI

- [x] Make CLI
- [x] Add commands:
  - [x] `zipity init`
  - [ ] `zipity export`
  - [x] `zipity serve`
  - [x] `zipity add <route | api> <name>`
- [ ] Add tests
- [ ] Add docs

## Getting Started

Create your project:

```bash
./cli/zipity init <PROJECT_NAME>
```

## Usage

```bash
./cli/zipity <SUBCOMMAND> [OPTIONS]
```

## Help

```text
Zipity 0.0.1-alpha.1
Eric David Smith
A command-line tool for Zipity

USAGE:
    cli [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add      Adds a new component <route | api> to the project
    help     Prints this message or the help of the given subcommand(s)
    init     Creates a new project
    serve    Serves the project locally for development
```
