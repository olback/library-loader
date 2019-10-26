# Library Loader :books:

![Screenshot](libloader.png)

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
2. Download a prebuilt version of library-loader from the [releases page](https://github.com/olback/library-loader/releases).
3. Generate a config file to store your settings in.  
Option 1: Generate a global config `library-loader -gh`  
Option 2: Generate a local config `library-loader -g` (current working directory).
4. Edit the config file to your liking. Global config locations:  
Windows: `C:\Users\%user%\AppData\Roaming\LibraryLoader.toml`  
Linux: `/home/$USER/.config/LibraryLoader.toml`  
Mac: `$HOME/Library/Preferences/LibraryLoader.toml`
5. Enjoy!

## Build from source
1. Make sure you have the rust toolchain installed.
2. Download the source. `git clone https://github.com/olback/library-loader.git`.
3. CD into `library-loader`.
4. Run `cargo build` to build, or `cargo run` to run.
5. The binary is stored in `target/[debug/release]/library-loader[.exe]`
6. Install by running `cargo install --path .`.

## What/Why?
This is an implementation of [https://www.samacsys.com/library-loader/](https://www.samacsys.com/library-loader/) in Rust. Why? Well, since the library-loader SamacSys provides only works on Windows, I thought it would be neat to make something similar but available to everyone.

For upcomming features, please see the [TODO.md](TODO.md).

## License
[GNU Affero General Public License v3.0](LICENSE)
