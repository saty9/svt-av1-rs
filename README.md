# SVT-AV1 bindings

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Actions Status](https://github.com/rust-av/svt-av1-rs/workflows/svt-av1/badge.svg)](https://github.com/rust-av/svt-av1-rs/actions)

It is a simple [binding][1] and safe abstraction over [SVT-AV1][2].

## Building

By default the bindings are generated using the headers and libraries that ought to be present in the system.

If you want to build the svt-av1 library from source you will need to set these env variables:

* PKG_CONFIG_PATH=/PATH/TO/SVT-AV1/Build/linux/Debug
* CPATH=/PATH/TO/SVT-AV1/Source/API
* LIBRARY_PATH=/PATH/TO/SVT-AV1/Bin/Debug
* LD_LIBRARY_PATH=/PATH/TO/SVT-AV1/Bin/Debug

these assume you are using a debug build of the library

###Requirements linux

* libclang
* llvm

## TODO
- [x] Simple bindings
- [ ] Safe abstraction
- [ ] Examples

[1]: https://github.com/rust-lang/rust-bindgen
[2]: https://github.com/OpenVisualCloud/SVT-AV1
