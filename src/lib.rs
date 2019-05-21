pub mod comm;
pub mod kv;

pub use comm::EtcdClientError;
pub use kv::{SimpleKV, SimpleKvClient};

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
