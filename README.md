etcdv3client-rust
======

NOTE: `etcdv3client` currently under early development, you should NOT use it in production environment.

[![Crates.io](https://img.shields.io/crates/v/etcdv3client)](https://crates.io/crates/etcdv3client)
[![Documentation](https://docs.rs/etcdv3client/badge.svg)](https://docs.rs/etcdv3client)
[![Build Status](https://dev.azure.com/kuwater0658/etcdv3client-rust/_apis/build/status/zzzdong.etcdv3client-rust?branchName=master)](https://dev.azure.com/kuwater0658/etcdv3client-rust/_build/latest?definitionId=1&branchName=master)
[![Crates.io](https://img.shields.io/crates/l/etcdv3client)](LICENSE)

## Overview

[`etcdv3client`] is a simple etcdv3 client in Rust-lang.

## Getting Started

Examples can be found in [`examples`].

## Support APIs

- [x] KV
- [x] Watch
- [ ] Lease
- [ ] Cluster
- [ ] Maintenance
- [ ] Auth

## Rust Version

`etcdv3client` currently works on rust `1.39` and above as it requires support for the `async_await`
feature.

## License

This project is licensed under the [MIT license](LICENSE).
