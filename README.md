# 🐱 NekoSource

A lightweight, cross-platform desktop Git client — written in pure Rust.

## Features

- **Cross-platform** - native builds for Windows, macOS, and Linux.
- **Localization** - English, Russian, Ukrainian, and Belarusian.
- **Easter eggs** - seasonal UI tweaks and a background soundtrack. (Soon™)

## Tech Stack

| Crate                                              | Role                               |
|----------------------------------------------------|------------------------------------|
| [`GPUI Kit`](https://github.com/longbridge/gpui-kit) | GPUI desktop UI and native components |
| [`git2`](https://github.com/rust-lang/git2-rs)     | libgit2 bindings — the Git engine  |

## Building

Install the Rust toolchain and platform build tools (MSVC/Windows SDK on Windows,
Xcode command-line tools on macOS, or the GPUI Linux development dependencies).
See [GPUI Kit installation](https://gpui-kit.com/docs/installation/) for platform setup.
Git must be in `PATH` for cloning and pushing; GnuPG is required for GPG signing.

```bash
git clone https://github.com/OctoBanon-Main/nekosource.git
cd nekosource
cargo build --release
```

The binary lands in `target/release/`.

## Credits

**Ukrainian translation & belarusian translation** - [ShadowCj](https://github.com/bluzetX)  
**Belarusian translation** - [xelframe](https://github.com/xelframe)

## License

This project is licensed under the MIT license.
