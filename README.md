# IRTUI

A terminal user interface for the [Neal.fun Internet Roadtrip](https://neal.fun/internet-roadtrip)

## Features

## Demo

![IRTUI Demo](/assets/demo/demo.gif)

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

If you ever need to see the logs, just set `IRTUI_LOG_LEVEL` to one of `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`, which correspond to the tracing log levels, listed from most to least verbose. Logs will be outputted in your system's default log directory:
- MacOS: `~/Library/Logs/irtui.log`
- Linux: `~/.local/share/irtui/log/irtui.log`
- Windows: `~\Local\irtui\logs\irtui.log`

## Features/Wishlist

- [ ] Add chat box 
- [ ] Support voting
- [ ] Display minimap
- [ ] Display and play the radio
- [ ] Add a link to the main site

## License

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
