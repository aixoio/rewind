# rewind
> a simple git wrapper built for speed

rewind is a git wrapper made in Rust, built for
speed, with the goal of providing people with a
simple tool to use with git. rewind is best used
along with git so that you can still get the full
set of features from git while also getting the
speed provided by rewind.

## Installing
Rewind installs the `rw` command on to your system.

### Prerequisites
You will need the current version of the Rust toolchain
to install rewind.

- Rust toolchain
- Cargo

These can both be installed with the `rustup` tool.

You will also need:
- Git

### Build from source

To install rewind (rw) first you will need to clone
the source code on to your machine
```bash
git clone https://github.com/aixoio/rewind.git
```
Once you have the source code for rewind on your
machine simply move into the rewind folder
```bash
cd rewind
```
Finaily to install rewind you will need to use
`cargo` run
```bash
cargo install --path .
```
Check it is installed with
```bash
rw --version
```

## Usage
Rewind offers many commands and sub-commands you can
use to manage your projects while being fully compatable
with git.

Help is offered with the `rw help` command.

### Commands

- `rw push` or `rw p` - push changes to origin
- `rw pull` or `rw pu` - pull changes from origin
- `rw status` or `rw s` - prints git status
- `rw add` or `rw ad` - stage files to git
- `rw commit` or `rw c` - commit files to git with auto staging
- `rw init` - runs `git init`
- `rw log` or `rw l` - display commit history
- `rw diff` or `rw d` - display `git diff` 
- `rw branch` or `rw b` - manage git branches
- `rw merge` or `rw m` - merge 2 branches
- `rw checkout` or `rw co` - checkout a branch or commit
- `rw reset` - discard all uncommited changes
- `rw tag` or `rw t` - manage git tags
- `rw revert` - revert changes made in a commit
- `rw stash` - manage stashes

## License
This project is under the MIT license.
