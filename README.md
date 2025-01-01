# Wyoming Protocol Library

A Rust implementation of the [Wyoming protocol](https://github.com/rhasspy/wyoming) - a peer-to-peer protocol designed for voice assistants.

## Overview

This library provides an abstraction layer for the Wyoming protocol, enabling seamless communication between voice assistant components. Wyoming is designed to facilitate peer-to-peer interactions in voice assistant systems.

## Goals

- Pure Rust implementation
- HTTP client independent, written in [Sans I/O](https://sans-io.readthedocs.io/how-to-sans-io.html#what-is-an-i-o-free-protocol-implementation) style
- Lightweight and efficient

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
wyoming = "0.1.0"
```