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

### `pu [--tags | -f]` – push, optionally including refs under refs/tags, or force push

Finally you push with `pu`.

`pu` calls `git push` with the optionally provided flag.
