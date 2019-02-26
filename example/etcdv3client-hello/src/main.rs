use etcdv3client::{EtcdClientError, EtcdV3Client};

fn main() {
    env_logger::init();

    let key = "hello";
    let value = "world";

    let host: &str = "127.0.0.1";
    let port: u16 = 2379;

    let etcd_client = EtcdV3Client::new(host, port).unwrap();

    let kv_client = etcd_client.new_simple_kv();

    match kv_client.get_string(key) {
        Ok(v) => {
            let value = format!("{}+", v);
            kv_client.put_string(key, &value).unwrap();
            println!("get {}: {}, put new: {}", key, v, value);
        }
        Err(EtcdClientError::KeyNotFound) => {
            kv_client.put_string(key, &value).unwrap();
            println!("cannot find {}", key);
        }
        Err(e) => {
            println!("etcd get failed, {:?}", e);
        }
    }
}
