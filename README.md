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
```rust
use etcdv3client::{EtcdClient, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let endpoint = "http://localhost:2379";
    let cred: Option<(String, String)> = None;
    let mut client = EtcdClient::new(vec![endpoint], cred).await?;

    let key = "/hello";
    // use convenience api under EtcdClient.
    match client.get(key).await {
        Ok(v) => {
            println!("got `{}` => {:?}", key, String::from_utf8_lossy(&v));
        }
        Err(err) => {
            if err.is_key_not_found() {
                 eprintln!("can not find `{}`", key);
            } else {
                 eprintln!("etcd get failed: `{:?}`", e);
            }
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

## Supported Rust Versions

 The current MSRV is 1.64.0.

## License

This project is licensed under the [MIT license](LICENSE).
