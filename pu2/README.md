# pu2

[![Crates.io](https://img.shields.io/crates/v/pu2.svg)](https://crates.io/crates/pu2)

This crate provides the `pu` command, and is part of the
[repotools](https://crates.io/crates/repotools) set of utilities.

You can install all of the repotools utilites by running
the following command in your terminal:

```bash
cargo install repotools
```

If you want to install only some of them, you can install them
individually. Refer to the [repotools](https://crates.io/crates/repotools)
for the full list of utilities and links to their individual crates.

To install `pu` utility individually, run the following
command in your terminal:

```bash
cargo install pu2
```

## Usage

### `pu [--tags] [-f] [<repository>]` – push commits or tags to remote git repository

Use `pu` to push commits to the remote of the current branch
(or to *origin* if no remote is configured for the current branch).

Use `pu --tags` to push tags.

Use `pu -f` to force push commits.

Use `pu --tags -f` to force push tags.

For all of the above, optionally specify which repository (`[<repository>]`)
to push to, specified either as a named remote or as a URL.

`pu` is like `git push`, only more awesome.
