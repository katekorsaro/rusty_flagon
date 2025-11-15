# rusty_flagon

A Rust-based character generator for OSE Classic role-playing game.

## `rusty_flagon_cli`

This is a command-line tool to generate characters using the `rusty_flagon_lib`.

### Usage

You can generate a character and print it to the console or save it to a file.

#### Print to Console

To generate a character and display it in your terminal:

```sh
rusty_flagon_cli
```

or explicitly:

```sh
rusty_flagon_cli std-out
```

This will produce a colorful character sheet directly in your terminal.

#### Save to File

To generate a character and save it as a Markdown file:

```sh
rusty_flagon_cli file
```

This will create a file in the current directory with a name like `character-name_(class).md`. For example: `gandalf_(magic_user).md`. The content of the file will be a Markdown-formatted character sheet.
