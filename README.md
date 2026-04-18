# IRTUI

A terminal user interface for the [Neal.fun Internet Roadtrip](https://neal.fun/internet-roadtrip)

## Features

## Demo

![IRTUI Demo](/assets/demo/demo.gif)

Actual results may vary, based on truecolor and emoji support of your terminal emulator. I used the VsCode default terminal on macOS Tahoe.

Sorry for the interstate 😭, I hate interstates too, btw, AIM for life!!

## Getting Started

### Prebuilt binaries
Prebuilt binaries are available in the [releases section](https://github.com/lazo4/irtui/releases/latest)

For now, no binaries are available, but I will try to make as many as possible

### Build from source

#### Prerequisites

- [Install Rust](https://rustup.rs)

##### MacOS and Linux
- Install [chafa](https://hpjansson.org/chafa/download/)

##### Windows

Chafa is currently unsupported on Windows due to pkg-config issues. If you know how to make it work, please
submit a PR.

#### Building

Now clone the repo:
```bash
git clone https://github.com/lazo4/irtui && cd irtui
```

Now you can just run: 
```bash
cargo build --release
```

Or, if you want to install it locally: 
```bash
cargo install --path .
```

## Usage

Just run it:
```bash
irtui
```

### Logging

If you ever need to see the logs, just set `IRTUI_LOG_LEVEL` to one of `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`, which correspond to the `tracing-subscriber` log levels, listed from most to least verbose. Logs will be outputted in your system's default log directory:
- Unix: `~/.local/share/irtui/log/irtui.log`
- Windows: `~\Local\irtui\logs\irtui.log`

Warning: the `TRACE` and `DEBUG` levels are very verbose and can quickly take up several GB of storage, beware.

## Features/Wishlist

- [ ] Support honking
- [ ] Add chat box 
- [ ] Support voting
- [ ] Display the odometer
- [ ] Display the minimap
- [ ] Display and play the radio
- [ ] Add a link to the main site and to the discord
- [ ] Maybe support [custom glyphs](https://rapha.land/introducing-glyph-protocol-for-terminals/), for the vote options icons

## License

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
