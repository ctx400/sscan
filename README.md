# sscan - A scriptable file/process/network scanner

[![Crates.io Version](https://img.shields.io/crates/v/sscan)](https://crates.io/crates/sscan)
[![docs.rs](https://img.shields.io/docsrs/sscan)](https://docs.rs/sscan/latest/sscan/)
[![GitHub last commit](https://img.shields.io/github/last-commit/ctx400/sscan)](https://github.com/ctx400/sscan/commits/main/)


**sscan** is a scriptable file, process, and network scanner.
Its high level of configurability is powered by userscripts which run in
an embeded [Lua](https://www.lua.org/) virtual machine.

Currently, scanning is provided by the
[YARA-X](https://virustotal.github.io/yara-x/) scan engine. YARA-X is a
Rust implementation of the original YARA scan engine. Additional scan
engines may be implemented or integrated in the future.

The embedded Lua virtual machine is made possible by the
[mlua](https://crates.io/crates/mlua) crate.

## *Early Development!*

This crate is in *very* early development. While I expect sscan to be
totally epic once stable, it is almost useless in its current state.

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

## Roadmap

- Full configurability with Lua userscripts
- YARA-X scan engine integration
- Custom Lua userscript scan engines

## Open-Source Licenses

sscan is made free and open-source to the public in accordance with
the terms of the [MIT License](LICENSE.md).

sscan uses third-party open-source software. A list of dependencies and
attribution information is available in
[OPEN_SOURCE_LICENSES.html](OPEN_SOURCE_LICENSES.html). You can view
this file as a rendered webpage
[here](https://htmlpreview.github.io/?https://github.com/ctx400/sscan/blob/main/OPEN_SOURCE_LICENSES.html).

Finally, sscan uses an embedded Lua virtual machine through the crate
`mlua` This ultimately uses the [Lua](https://www.lua.org/) software.
The license for Lua can be found in [LUA_LICENSE.md](LUA_LICENSE.md).

A good-faith effort is made to ensure all dependences are properly
attributed. I use both the tools `cargo-deny` and `cargo-about` to scan
for open-source license requirements. However, if you are a crate author
and need to update OPEN_SOURCE_LICENSES.html, please open an issue using
the **Attribution Issues** template.
