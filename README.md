# libhackrf-rs <img src="https://img.shields.io/badge/maintenance-stable-green" alt="Maintenance"> <img src="https://img.shields.io/badge/license-MIT-blue" alt="License"> <a href="https://discordapp.com/users/346979343995633664/"><img src="https://img.shields.io/badge/chat-on_discord-%237289DA" alt="Discord"></a>
Rust API for the HackRF One software defined radio (SDR).

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
  - 11 22H2 :white_check_mark:

If you have got a desktop pc/laptop with Mac OS, I would appreciate your feedback about compatibility.

## Build and run quick-guide
Building project:
```sh
cargo build # or with --release argument
```
Running info binary example (after project build):
```sh
cargo run --package libhackrf-rs --bin info # you can also change it to rx or tx
```
## Use library as dependency
Add this line to your `Cargo.toml`:
```toml
libhackrf-rs = { git = "https://github.com/fl1ckje/libhackrf-rs", branch = "master" }
```
If you don't need to build binary examples, you may use this [no-examples-branch]:
```toml
libhackrf-rs = { git = "https://github.com/fl1ckje/libhackrf-rs", branch = "no-examples" }
```
or just comment out this stuff in `Cargo.toml`
```toml
[[bin]]
name = "info"
path = "src/examples/info.rs"

[[bin]]
name = "rx"
path = "src/examples/rx.rs"

[[bin]]
name = "tx"
path = "src/examples/tx.rs"
```

[rusb]: https://github.com/a1ien/rusb
[HackRF One]: https://greatscottgadgets.com/hackrf/one/
[libhackrf]: https://github.com/greatscottgadgets/hackrf/tree/master/host
[no-examples-branch]: https://github.com/fl1ckje/libhackrf-rs/tree/no-examples
