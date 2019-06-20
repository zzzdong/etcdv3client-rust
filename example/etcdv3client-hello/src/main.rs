use etcdv3client::{EtcdClientError, EtcdV3Client};

fn main() {
    env_logger::init();

    let key = "hello";
    let value = "world";

    let host: &str = "127.0.0.1";
    let port: u16 = 2379;

    let etcd_client = EtcdV3Client::new(host, port).expect("can not connect etcd server");

    let kv_client = etcd_client.new_simple_kv();

    match kv_client.get_string(key) {
        Ok(v) => {
            println!("got {}: {}", key, v);
            let value = format!("{}+", v);
            kv_client.put(key, &value).expect("put failed");
            println!("put new: {}", value);
        }
        Err(EtcdClientError::KeyNotFound) => {
            println!("cannot find `{}`", key);
            println!("try to put {} with {}", key, value);
            kv_client.put(key, value).expect("put failed");
        }
        Err(e) => {
            println!("etcd get failed, {:?}", e);
        }
    }

    let kvs = kv_client
        .get_with_prefix("h")
        .expect("get_with_prefix failed");
    // let kvs = kv_client.get_all().expect("get_all failed");

    println!("{:?}", kvs);

    // for kv in &kvs {
    //     println!("{:?}", kv);
    // }
}
