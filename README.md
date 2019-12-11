# repotools

[![Crates.io](https://img.shields.io/crates/v/repotools.svg)](https://crates.io/crates/repotools) [![GitHub stars](https://img.shields.io/github/stars/ctsrc/repotools?style=social)](https://github.com/ctsrc/repotools#start-of-content)

Short names, big time savings – a collection of commands
for the git operations you perform most often.

Meaningful and memorable: All of the commands have names that,
even though short, make sense... For the most part ;)

The commands are, in short order: `dp`, `aa`, `st`, `di`, `cm`, `pu` and `le`.

These correspond to `git diff`, `git add -A`, `git status`, `git diff --cached`, `git commit -m <message>`, `git push` and `git shortlog -se`.

Fuller descriptions of the commands are availble in the usage section below.
Read on to get the details, or [skip straight to installation](#installation)
if you are feeling too impatient. (But do go back and read how to use them
afterwards if you end up installing them right away.)

## Usage

### `dp [<path>...]` – diff files pending staging in git repository, optionally limited to one or more files or directories

When you look at what changes you want to stage for commit, you type `dp`.

`dp` is like `git diff`, only more awesome.

### `aa [<path>...]` – add all files in git repository to index, optionally limited to one or more directories

When you want to stage everything, or everything in certain directories,
for commit, you type `aa` or `aa <path>...` respectively.

`aa` is like `git add -A`, only more awesome.

### `st [<pathspec>...]` – status of git repository, optionally limited to one or more files or directories

My workflow usually goes, write some code, stage some code, research something,
test the changes, write some code, stage some code, and so on.

Every now and then between writing, staging, etc, I look at what
I've staged for commit so far, and what has not yet been staged.
For the high-level bird's eye view of that there is `st`.

`st` is like `git status`, only more awesome.

### `di [<path>...]` – diff files staged for commit in git repository, optionally limited to one or more files or directories

Prior to committing, and also now and then while still working on the code,
it is very useful to review what you are about commit, both to ensure that
you are about to commit what you think you are about to commit, and in order to
write useful commit messages that accurately describe the changes.

`di` shows the diff for the staged changes against HEAD.

`di` is like `git diff --cached`, only more awesome.

### `cm <message>` – git commit with message

When it comes time to commit, you have `cm`.

`cm` is like `git commit -m`, only more awesome.

### `pu [--tags] [-f] [<repository>]` – push commits or tags to remote git repository

Finally you push with `pu`.

Use `pu` to push commits to the remote of the current branch
(or to *origin* if no remote is configured for the current branch).

Use `pu --tags` to push tags.

Use `pu -f` to force push commits.

Use `pu --tags -f` to force push tags.

For all of the above, optionally specify which repository (`[<repository>]`)
to push to, specified either as a named remote or as a URL.

`pu` is like `git push`, only more awesome.

### `le` – number of commits by each author in git repository

Among all of the commands, this is the only one that doesn't match
the meaning with its name. I would have named this command as I initially
did; `nc`, if it wasn't for the fact that the netcat command on a lot
of systems is named `nc` already.

The name comes from "l" for "log" and "e" for the `-e` flag.

`le` is like `git shortlog -se`, only more awesome.

## Installation

### Precompiled binaries

Precompiled binaries are available for the following operating systems:

* macOS Catalina 10.15.1 x86_64
* Ubuntu 18.04 Bionic GNU/Linux x86_64
* KDE neon User Edition 5.17 GNU/Linux x86_64
* FreeBSD 12.1 x86_64

Download links and installation instructions are provided below.

#### macOS Catalina 10.15.1 x86_64

Download: [repotools-v0.7.2-macOS-Catalina-x86_64.tbz](https://github.com/ctsrc/repotools/releases/download/repotools-v0.7.2/repotools-v0.7.2-macOS-Catalina-x86_64.tbz)

Installation:

```sh
cd ~
curl -LJO https://github.com/ctsrc/repotools/releases/download/repotools-v0.7.2/repotools-v0.7.2-macOS-Catalina-x86_64.tbz
tar xvf repotools-v0.7.2-macOS-Catalina-x86_64.tbz
```

The binaries are now available in `~/bin/`. Add that directory
to your `$PATH` if you don't have it in there already, and then
if you didn't have it there already, log out and log in again.

#### Ubuntu 18.04 Bionic GNU/Linux x86_64

Download: [repotools-v0.7.2-ubuntu-bionic-x86_64.txz](https://github.com/ctsrc/repotools/releases/download/repotools-v0.7.2/repotools-v0.7.2-ubuntu-bionic-x86_64.txz)

Being based on Ubuntu 18.04 Bionic, KDE neon User Edition 5.17 and the former
share the same binary release file. You should be able to use the same binaries
on any other GNU/Linux distro that is based on Ubuntu 18.04 Bionic as well.

Installation:

```sh
cd ~
wget https://github.com/ctsrc/repotools/releases/download/repotools-v0.7.2/repotools-v0.7.2-ubuntu-bionic-x86_64.txz
tar xvf repotools-v0.7.2-ubuntu-bionic-x86_64.txz
```

The binaries are now available in `~/bin/`. Add that directory
to your `$PATH` if you don't have it in there already, and then
if you didn't have it there already, log out and log in again.

#### KDE neon User Edition 5.17 GNU/Linux x86_64

Download: [repotools-v0.7.2-ubuntu-bionic-x86_64.txz](https://github.com/ctsrc/repotools/releases/download/repotools-v0.7.2/repotools-v0.7.2-ubuntu-bionic-x86_64.txz)

Being based on Ubuntu 18.04 Bionic, KDE neon User Edition 5.17 and the former
share the same binary release file. You should be able to use the same binaries
on any other GNU/Linux distro that is based on Ubuntu 18.04 Bionic as well.

Installation:

```sh
cd ~
wget https://github.com/ctsrc/repotools/releases/download/repotools-v0.7.2/repotools-v0.7.2-ubuntu-bionic-x86_64.txz
tar xvf repotools-v0.7.2-ubuntu-bionic-x86_64.txz
```

The binaries are now available in `~/bin/`. Add that directory
to your `$PATH` if you don't have it in there already, and then
if you didn't have it there already, log out and log in again.

#### FreeBSD 12.1 x86_64

Download: [repotools-v0.7.2-freebsd-12.1-x86_64.txz](https://github.com/ctsrc/repotools/releases/download/repotools-v0.7.2/repotools-v0.7.2-freebsd-12.1-x86_64.txz)

Installation:

```sh
cd ~
fetch https://github.com/ctsrc/repotools/releases/download/repotools-v0.7.2/repotools-v0.7.2-freebsd-12.1-x86_64.txz
tar xvf repotools-v0.7.2-freebsd-12.1-x86_64.txz
```

The binaries are now available in `~/bin/`. Add that directory
to your `$PATH` if you don't have it in there already, and then
if you didn't have it there already, log out and log in again.

### Building from source

In order to build from source, you need to have the Rust toolchain installed.
If you don't have the Rust toolchain installed already, head on over to
https://rustup.rs/ and follow the instructions there. Then come back here
once you've done that.

With the Rust toolchain installed, simply run:

```bash
cargo install repotools
```

This will build and install all of the repotools command utilities.

#### Building and installing only some of the repotools command utilites

If you only wish to build and install some of the repotools command utilities
rather than all of them, you can do so by making use of the feature flags.

With the Rust toolchain installed (see above), run the following command, with
the features list adjusted according to your wishing with regards to which of
the command utilities you would like to build and install:

```bash
cargo install repotools --no-default-features \
  --features "dp aa st di cm pu le"
```

## Star Me on GitHub 🤩

Do you think this project is awesome? I think *you* are awesome!
Now show me some love and
[star this project on GitHub](https://github.com/ctsrc/repotools#start-of-content).

[![GitHub stars](https://img.shields.io/github/stars/ctsrc/repotools?style=social)](https://github.com/ctsrc/repotools#start-of-content)
