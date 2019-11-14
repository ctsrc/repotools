# dp2

[![Crates.io](https://img.shields.io/crates/v/dp2.svg)](https://crates.io/crates/dp2)

This crate provides the `dp` command, and is part of the
[repotools](https://crates.io/crates/repotools) set of utilities.

You can install all of the repotools utilites by running
the following command in your terminal:

```bash
cargo install repotools
```

If you want to install only some of them, you can install them
individually. Refer to the [repotools](https://crates.io/crates/repotools)
for the full list of utilities and links to their individual crates.

To install `dp` utility individually, run the following
command in your terminal:

```bash
cargo install dp2
```

## Usage

### `dp [<path>...]` – diff pending, optionally limited to one or more files or directories

When you look at what changes you want to stage for commit, you type `dp`.

`dp` is like `git diff`, only more awesome.
