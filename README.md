# rmbuild

A tiny tool for removing build folders

## Motivation

Sometimes you need to backup folders that contains many source code written by yourself.

The source codes are just text files which is very easy to move, but the build folders contains many binaries which are usually large and takes quite a long time to move.

This little command line tool is to help you remove these build folders quickly so that you don't need to worry about the size and the amount of these folders.

## Usage

> Note: If you don’t have Rust yet, I recommend you use [`rustup`](https://rustup.rs/) to manage your Rust installation.
[The official rust guide](https://doc.rust-lang.org/book/ch01-01-installation.html) has a wonderful section on getting started.

First, you need to install the binaries from source code:

```bash
git clone https://github.com/Azathoth1729/rmbuild
cd rmbuild
cargo install --path .
```

Now you should be able to run commands `rmbuild` directly on your terminal:

```bash
rmbuild 0.1.0
Azathoth
A tiny tool for removing build folders

USAGE:
    rmbuild [PATH]

ARGS:
    <PATH>    starting path for rmbuild, default is your home directory

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## Uninstalling

Simply run if you install the binary from `cargo install`

```bash
cargo uninstall pngchat
```

## TODO

+ Add support for other languages's build targets
+ Add tests for the correctness of program
