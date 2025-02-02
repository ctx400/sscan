# sscan - A scriptable file/process/network scanner

[![Crates.io Version](https://img.shields.io/crates/v/sscan)](https://crates.io/crates/sscan)
[![docs.rs](https://img.shields.io/docsrs/sscan)](https://docs.rs/sscan/latest/sscan/)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/ctx400/sscan/.github%2Fworkflows%2Frust.yml)](https://github.com/ctx400/sscan/actions/workflows/rust.yml)
[![GitHub last commit](https://img.shields.io/github/last-commit/ctx400/sscan)](https://github.com/ctx400/sscan/commits/main/)


**sscan** is a scriptable file, process, and network scanner.
Its high level of configurability is powered by userscripts which run in
an embeded [Lua](https://www.lua.org/) virtual machine.

Scanning is provided via both custom, user-defined scan engines, as well
as a built-in YARA scan engine (provided by YARA-X, currently WIP.)
A global scan queue is implemented to automatically distribute files and
other scannable data to all activated scan engines.

The embedded Lua virtual machine is made possible by the
[mlua](https://crates.io/crates/mlua) crate.

## *Early Development!*

This crate is in early development. Some features are available now,
such as custom userscript scan engine support, but other features are
still in progress!

## Getting Started

To install sscan, run the following in your terminal:

```bash
cargo install --locked sscan
```

To try out sscan in interactive mode, run:

```bash
sscan interactive
```

Or, if you have already created a Lua userscript, run:

```bash
sscan run myscript.lua
```

sscan has a built-in help system. You can access it by calling (in Lua):

```lua
help()            -- For general help, or
help:topics()     -- For a list of specific help topics, or
help 'topic_name' -- To view detailed help on a topic.
```

Finally, you can get even more help from [the docs](https://docs.rs/sscan/latest/sscan).

## Free and Open-Source Software

sscan is made free and open-source to the public in accordance with
the terms of the [MIT License](LICENSE.md).

sscan uses third-party open-source software. A list of dependencies and
attribution information is available in
[OPEN_SOURCE_LICENSES.html](OPEN_SOURCE_LICENSES.html). You can view
this file as a rendered webpage
[here](https://htmlpreview.github.io/?https://github.com/ctx400/sscan/blob/main/OPEN_SOURCE_LICENSES.html).

A good-faith effort is made to ensure all dependences are properly
attributed. I use both the tools `cargo-deny` and `cargo-about` to scan
for open-source license requirements. However, if you are a crate author
and need to update OPEN_SOURCE_LICENSES.html, please open an issue using
the **Attribution Issues** template.

## Versioning

sscan tries to follow SemVer 2.0:

- Breaking changes will receive a major version bump.
- New functionality will receive a minor version bump.
- Bug fixes receive a patch bump, unless they are breaking changes,
  in which case they will be included in the next major release.

Branch `main` is bleeding-edge. If you want to experience the latest and
greatest features *at risk of instability*, build from `main`. Features
are developed on separate branches then merged into `main`.

A branch will be created for each point release. To checkout a specific
version of sscan, use `git checkout vX.Y.Z`

Finally, any versions \< v1.0.0 are early development. Consider them unstable!
In these versions, *anything may change at any time*!
