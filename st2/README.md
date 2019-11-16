# st2

[![Crates.io](https://img.shields.io/crates/v/st2.svg)](https://crates.io/crates/st2)

This crate provides the `st` command, and is part of the
[repotools](https://crates.io/crates/repotools) set of utilities.

You can install all of the repotools utilites by running
the following command in your terminal:

```bash
cargo install repotools
```

If you want to install only some of them, you can install them
individually. Refer to the [repotools](https://crates.io/crates/repotools)
for the full list of utilities and links to their individual crates.

To install `st` utility individually, run the following
command in your terminal:

```bash
cargo install st2
```

## Usage

### `st [<pathspec>...]` – status of git repository, optionally limited to one or more files or directories

My workflow usually goes, write some code, stage some code, research something,
test the changes, write some code, stage some code, and so on.

Every now and then between writing, staging, etc, I look at what
I've staged for commit so far, and what has not yet been staged.
For the high-level bird's eye view of that there is `st`.

`st` is like `git status`, only more awesome.
