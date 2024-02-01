# libhackrf-rs
![Maintenance](https://img.shields.io/badge/maintenance-stable-green.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Discord](https://img.shields.io/badge/chat-on_discord-%237289DA.svg)
[![build workflow](https://github.com/fl1ckje/libhackrf-rs/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/fl1ckje/libhackrf-rs/actions/workflows/build.yml)

Rust API for the [HackRF One] software defined radio (SDR).
It's a reimplementation of [libhackrf] in Rust using a safe [rusb] wrapper.

At the current time, this library can:
* provide firmware and board info;
* set parameters of SDR;
* receive data;
* transmit data.

For full feature support use the official C library.

## Supported operating systems
Library runs on both Linux and Windows:
* Linux:
  - Ubuntu 22.04 :white_check_mark:
* Windows:
  - 7 :white_check_mark:
  - 8 :white_check_mark:
  - 8.1 :white_check_mark:
  - 10 :white_check_mark:
  - 11 :white_check_mark:

If you have got a desktop pc/laptop with Mac OS, I would appreciate your feedback about compatibility.

## Build and run quick-guide
Building project:
```sh
$ cargo build # or with --release argument
```
## Use library as dependency
Add this line to your `Cargo.toml`:
```toml
libhackrf-rs = { git = "https://github.com/fl1ckje/libhackrf-rs", branch = "master" }
```

[rusb]: https://github.com/a1ien/rusb
[HackRF One]: https://greatscottgadgets.com/hackrf/one/
[libhackrf]: https://github.com/greatscottgadgets/hackrf/tree/master/host
