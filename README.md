[![crates.io][crates-badge]][crates-url] [![docs.rs][docs-badge]][docs-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/driver-74hc595
[crates-url]: https://crates.io/crates/driver-74hc595
[docs-badge]: https://docs.rs/driver-74hc595/badge.svg
[docs-url]: https://docs.rs/driver-74hc595
[actions-badge]: https://github.com/tactile-eng/driver-74hc595/workflows/CI/badge.svg
[actions-url]: https://github.com/tactile-eng/driver-74hc595/actions?query=workflow%3ACI+branch%3Amain

# driver-74hc595 - Embedded async 74hc595 driver

<!-- cargo-rdme start -->

An embedded async driver for 74hc595 (compatible) shift registers.

For efficient data transfer, an SPI bus peripheral is used. The SPI bus should be configured in SPI Mode 0 (clock
idle state low, data sampled on rising edge). Because the 74hc595 does not have a CS line, the SPI bus cannot be
shared with other devices (at least not without additional circuitry).

<!-- cargo-rdme end -->
