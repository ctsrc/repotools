# di2

[![Crates.io](https://img.shields.io/crates/v/di2.svg)](https://crates.io/crates/di2)

This crate provides the `di` command, and is part of the
[repotools](https://crates.io/crates/repotools) set of utilities.

You can install all of the repotools utilites by running
the following command in your terminal:

```bash
cargo install repotools
```

If you want to install only some of them, you can install them
individually. Refer to the [repotools](https://crates.io/crates/repotools)
for the full list of utilities and links to their individual crates.

To install `di` utility individually, run the following
command in your terminal:

```bash
cargo install di2
```

## Usage

### `di [<path>...]` – diff staged, optionally limited to one or more files or directories

Prior to committing, and also now and then while still working on the code,
it is very useful to review what you are about commit, both to ensure that
you are about to commit what you think you are about to commit, and in order to
write useful commit messages that accurately describe the changes.

`di` shows the diff for the staged changes against HEAD.

`di` calls `git diff --cached` with the optionally provided argument.
