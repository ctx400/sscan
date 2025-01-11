# sscan - A scriptable file/process/network scanner

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
