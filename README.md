![Maintenance](https://img.shields.io/badge/maintenance-stable-green)

# libhackrf-rs
Rust API for the HackRF One software defined radio

This is a reimplementation of [libhackrf] `libhackrf` in Rust using the [rusb] `libusb` wrapper.
It lacks some features, however it can still provide firmware and board info, receive and transmit data.
For full feature support use the official C library.

## Operation system support
This library supports both Linux and Windows:
* Linux:
  - Ubuntu 22.04
* Windows:
  - 11 22H2
If you have got a desktop pc/laptop with Mac OS, I would appreciate your feedback about compatibility.

[rusb]: https://github.com/a1ien/rusb
[HackRF One]: https://greatscottgadgets.com/hackrf/one/
[libhackrf]: https://github.com/greatscottgadgets/hackrf/tree/master/host
