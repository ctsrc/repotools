# repotools

[![Crates.io](https://img.shields.io/crates/v/repotools.svg)](https://crates.io/crates/repotools) [![GitHub stars](https://img.shields.io/github/stars/ctsrc/repotools?style=social)](https://github.com/ctsrc/repotools#start-of-content)

Short names, big time savings – a collection of commands
for the git operations you perform most often.

Meaningful and memorable: All of the commands have names that,
even though short, make sense... For the most part ;)

The commands are, in short order: `dp`, `aa`, `st`, `di`, `cm`, `pu` and `le`.

Read on below to see what they mean and what arguments they take, or
[skip straight to installation](#installation) if you are feeling too impatient.
(But do go back and read how to use them afterwards if you end up installing
them right away.)

## Usage

### `dp [<path>...]` – diff pending, optionally limited to one or more files or directories

When you look at what changes you want to stage for commit, you type `dp`.

`dp` is like `git diff`, only more awesome.

### `aa [<path>...]` – add all files in git repository to index, optionally limited to one or more directories

When you want to stage everything, or everything in certain directories,
for commit, you type `aa` or `aa <path>...` respectively.

`aa` is like `git add -A`, only more awesome.

### `st [<pathspec>...]` – status, optionally limited to one or more files or directories

My workflow usually goes, write some code, stage some code, research something,
test the changes, write some code, stage some code, and so on.

Every now and then between writing, staging, etc, I look at what
I've staged for commit so far, and what has not yet been staged.
For the high-level bird's eye view of that there is `st`.

`st` is like `git status`, only more awesome.

### `di [<path>...]` – diff staged, optionally limited to one or more files or directories

Prior to committing, and also now and then while still working on the code,
it is very useful to review what you are about commit, both to ensure that
you are about to commit what you think you are about to commit, and in order to
write useful commit messages that accurately describe the changes.

`di` shows the diff for the staged changes against HEAD.

`di` is like `git diff --cached`, only more awesome.

### `cm <message>` – commit with message

When it comes time to commit, you have `cm`.

`cm` is like `git commit -m`, only more awesome.

### `pu [--tags | -f]` – push, optionally including refs under refs/tags, or force push

Finally you push with `pu`.

`pu` is like `git push`, only more awesome.

### `le` – number of commits

Among all of the aliases, this is the only one that doesn't match
the meaning with its name. I would have named this alias as I initially
did; `nc`, if it wasn't for the fact that the netcat command on a lot
of systems is named `nc` already. And when I used aliases, I didn't feel
like having to remember to type `\nc` when I wanted netcat, nor did I
and nor do I feel like typing out the full path to netcat either, e.g.
`/usr/bin/nc`. So I took the "l" for "log" and the "e" for the `-e` flag.

`le` is like `git shortlog -se`, only more awesome.

## Installation

Precompiled binaries will be provided in the future, but for now you'll need
to have the Rust toolchain installed. If you don't have the Rust toolchain
installed already, head over to https://rustup.rs/ and follow the instructions
there and then come back here.

Once you have the Rust toolchain installed, simply run:

```bash
cargo install repotools
```

## Star Me on GitHub 🤩

Do you think this project is awesome? I think *you* are awesome!
Now show me some love and
[star this project on GitHub](https://github.com/ctsrc/repotools#start-of-content).

[![GitHub stars](https://img.shields.io/github/stars/ctsrc/repotools?style=social)](https://github.com/ctsrc/repotools#start-of-content)
