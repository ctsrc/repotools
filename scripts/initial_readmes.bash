#!/usr/bin/env bash

for crate in *2 ; do
  command=$( echo $crate | sed 's/;$//')
  cat >"$crate/README.md" <<EOF
# $crate

[![Crates.io](https://img.shields.io/crates/v/$crate.svg)](https://crates.io/crates/$crate)

This crate provides the \`$command\` command, and is part of the
[repotools](https://crates.io/crates/repotools) set of utilities.

You can install all of the repotools utilites by running
the following command in your terminal:

\`\`\`bash
cargo install repotools
\`\`\`

If you want to install only some of them, you can install them
individually. Refer to the [repotools](https://crates.io/crates/repotools)
for the full list of utilities and links to their individual crates.

To install \`$command\` utility individually, run the following
command in your terminal:

\`\`\`bash
cargo install $crate
\`\`\`

## Usage


EOF
done
