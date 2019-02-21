
use etcdv3client::EtcdV3Client;


fn main() {
    println!("Hello, world!");

    env_logger::init();

    let key = "hello";
    let value = "world";

    let host: &str = "127.0.0.1";
    let port: u16 = 2379;

    let etcd_client = EtcdV3Client::new(host, port).unwrap();

    let kv_client = etcd_client.new_kvclient();
    
    let value = kv_client.get_string(key).unwrap();
    let value = format!("{}+", value);

    kv_client.put_string(key, &value).unwrap();

    dbg!(&value);
}
