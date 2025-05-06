# rust_ls

A simple `ls`-like utility written in Rust. Supports common listing options such as long format, sorting by size or time, showing hidden files, and displaying file authors. 
This is not meant for actual use, and has not been tested throughly (at all)

## Features

- `-a`, `--all`: Show hidden files (those starting with `.`)
- `-l`: Long listing format
- `--author`: Show the author (user) in long format
- `-t`: Sort by modification time
- `-s`: Sort by file size
- `-r`: Reverse the sorting order
- Accepts a path argument (defaults to current directory)

## Cons
- no dynamic formating
- lacks options

## Usage

```sh
cargo run -- [OPTIONS] [PATH]
```
Or build and run it 

```sh
cargo build
rust_ls -- [OPTIONS] [PATH]
```
| Option        | Description                             |
| ------------- | --------------------------------------- |
| `-a`, `--all` | Do not ignore entries starting with `.` |
| `-l`          | Use a long listing format               |
| `--author`    | Show author (user) in long listing      |
| `-t`          | Sort entries by modification time       |
| `-s`          | Sort entries by file size               |
| `-r`          | Reverse the sorting order               |
| `PATH`        | Path to list (defaults to `.`)          |


MIT License — Copyright (c) 2025 Youssef Bahri

Permission is granted to use, copy, modify, and distribute this software for any purpose, with or without fee.

This software is provided "as is", without warranty of any kind.

