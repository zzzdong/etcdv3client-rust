etcdv3client-rust
======

[![Crates.io](https://img.shields.io/crates/v/etcdv3client)](https://crates.io/crates/etcdv3client)
[![Documentation](https://docs.rs/etcdv3client/badge.svg)](https://docs.rs/etcdv3client)
![Rust](https://github.com/zzzdong/etcdv3client-rust/workflows/Rust/badge.svg)
[![Crates.io](https://img.shields.io/crates/l/etcdv3client)](LICENSE)

## Overview

[`etcdv3client`] is a simple etcdv3 client in Rust-lang.

## Example

A basic example:
```rust,no_run
use etcdv3client::{EtcdClient, EtcdClientError};

#[tokio::main]
async fn main() -> Result<(), EtcdClientError> {
    let endpoint = "http://localhost:2379";
    let auth: Option<(String, String)> = None;
    let mut client = EtcdClient::new(vec![endpoint], auth).await?;

    let key = "/hello";
    // use convenience api under EtcdClient.
    match client.get(key).await {
        Ok(v) => {
            println!("got `{}` => {:?}", key, String::from_utf8_lossy(&v));
        }
        Err(EtcdClientError::KeyNotFound) => {
            eprintln!("can not find `{}`", key);
        }
        Err(e) => {
            eprintln!("etcd get error: `{:?}`", e);
        }
    }

    Ok(())
}
```

More examples can be found in [`examples`].

## Support APIs

- [x] KV
- [x] Watch
- [x] Lease
- [ ] Cluster
- [ ] Maintenance
- [ ] Auth

## Rust Version

`etcdv3client` currently works on rust `1.39` and above as it requires support for the `async_await`
feature.

## License

This project is licensed under the [MIT license](LICENSE).
