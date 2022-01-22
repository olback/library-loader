# Library Loader :books:

<!-- ![Screenshot](libloader.png) -->

Status: [![CircleCI](https://circleci.com/gh/olback/library-loader/tree/master.svg?style=svg)](https://circleci.com/gh/olback/library-loader/tree/master)

<!---
OS | Status
-- | ------
Linux | [![CircleCI](https://circleci.com/gh/olback/library-loader/tree/master.svg?style=svg)](https://circleci.com/gh/olback/library-loader/tree/master)
Windows | WIP
Mac | WIP
--->

## Getting started

1. Create an account on [componentsearchengine.com](https://componentsearchengine.com/) if you don't have one already.
2. Download a prebuilt version of library-loader from the [releases page](https://github.com/olback/library-loader/releases) (only linux builds available, see [#67](https://github.com/olback/library-loader/issues/67)).

### Building from source using Docker

This allows you to build without installing any dependencies on your machine.

```
docker run --volume=$(pwd):/home/circleci/project olback/rust-gtk-linux cargo build --release
```

### Building from source locally(macOS)

Required binaries: brew(from homebrew), rustc, cargo
You have to install rust via rustup and initialize it with rustup-init command.

```shell
./macos-compile.sh
```

## What/Why?

This is an implementation of [https://www.samacsys.com/library-loader/](https://www.samacsys.com/library-loader/) in Rust. Why? Well, since the library-loader SamacSys provides only works on Windows, I thought it would be neat to make something similar but available to everyone.

For upcomming features, please see the [TODO.md](TODO.md).

## License

[GNU Affero General Public License v3.0](LICENSE)
