# le2

[![Crates.io](https://img.shields.io/crates/v/le2.svg)](https://crates.io/crates/le2)

This crate provides the `le` command, and is part of the
[repotools](https://crates.io/crates/repotools) set of utilities.

You can install all of the repotools utilites by running
the following command in your terminal:

```bash
cargo install repotools
```

If you want to install only some of them, you can install them
individually. Refer to the [repotools](https://crates.io/crates/repotools)
for the full list of utilities and links to their individual crates.

To install `le` utility individually, run the following
command in your terminal:

```bash
cargo install le2
```

## Usage

### `le` – number of commits

Among all of the aliases, this is the only one that doesn't match
the meaning with its name. I would have named this alias as I initially
did; `nc`, if it wasn't for the fact that the netcat command on a lot
of systems is named `nc` already. And when I used aliases, I didn't feel
like having to remember to type `\nc` when I wanted netcat, nor did I
and nor do I feel like typing out the full path to netcat either, e.g.
`/usr/bin/nc`. So I took the "l" for "log" and the "e" for the `-e` flag.

`le` calls `git shortlog -se`.
