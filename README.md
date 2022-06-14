# xtend

## Description
xtend is a simple tool to help add or remove file extensions.

I started this tool because I needed to change the file extension for certain files,
so that another tool stops detecting them when I needed to test something and restore
the old extension when I'm done.

This is relatively simple for single files, but if you need many of them it gets a bit more tricky.
There are plain unix ways to do it, but those are hard to remember and more error prone.

With this tool I can just do something like this:
```sh
# assuming there exists a file named 'test.txt'
xtend toggle bak test.txt
```
and if the file is named `test.txt` it will get renamed to `test.txt.bak`,
if it's named `test.txt.bak` instead, it will get renamed to `test.txt`.

## Install

From binary release:
[https://github.com/hardliner66/xtend/releases](releases)

With cargo:
```sh
cargo install xtend
```

## Usage:

```sh
xtend 0.1.0
Simple tool to work with file extensions

USAGE:
    xtend <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add               Adds an extension to all found files
    help              Print this message or the help of the given subcommand(s)
    remove            Removes an extension from all found files
    toggle            Adds an extension when it's missing or removes it when it's present
    toggle-between    Toggles between two extensions
```

### xtend add
```sh
xtend-add 
Adds an extension to all found files

USAGE:
    xtend add <EXTENSION> <GLOB>

ARGS:
    <EXTENSION>    The extension to add to a file
    <GLOB>         Glob pattern to search for files

OPTIONS:
    -h, --help    Print help information
```

### xtend remove
```sh
xtend-remove 
Removes an extension from all found files

USAGE:
    xtend remove <GLOB> [EXTENSION]

ARGS:
    <EXTENSION>    The extension to be removed from a file. Removes any extension if not set
    <GLOB>         Glob pattern to search for files

OPTIONS:
    -h, --help    Print help information
```

### xtend toggle
```sh
xtend-toggle 
Adds an extension when it's missing or removes it when it's present

USAGE:
    xtend toggle <EXTENSION> <GLOB>

ARGS:
    <EXTENSION>    Extension to be toggled
    <GLOB>         Glob pattern to filter files

OPTIONS:
    -h, --help    Print help information
```

### xtend toggle-between
```sh
xtend-toggle-between 
Toggles between two extensions

USAGE:
    xtend toggle-between <EXTENSION1> <EXTENSION2> [GLOB]

ARGS:
    <EXTENSION1>    Extension 1
    <EXTENSION2>    Extension 2
    <GLOB>          Optional glob pattern to filter files

OPTIONS:
    -h, --help    Print help information
```

