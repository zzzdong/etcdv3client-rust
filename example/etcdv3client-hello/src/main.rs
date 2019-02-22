
use etcdv3client::{EtcdV3Client, EtcdClientError};


fn main() {
    env_logger::init();

    let key = "hello";
    let value = "world";

    let host: &str = "127.0.0.1";
    let port: u16 = 2379;

    let etcd_client = EtcdV3Client::new(host, port).unwrap();

    let kv_client = etcd_client.new_kvclient();
    
    match kv_client.get_string(key) {
        Ok(v) => {
            let value = format!("{}+", v);
            kv_client.put_string(key, &value).unwrap();
            println!("get {}: {}, put new: {}", key, v, value);
        }
        Err(EtcdClientError::KeyNotFound(k)) => {
            kv_client.put_string(key, &value).unwrap();
            println!("cannot find {}", k)
        }
        Err(e) => {
            println!("etcd get failed, {:?}", e);
        }
    }
}
