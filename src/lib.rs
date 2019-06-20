pub mod auth;
pub mod client;
pub mod kv;
pub mod pb;

pub use client::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
