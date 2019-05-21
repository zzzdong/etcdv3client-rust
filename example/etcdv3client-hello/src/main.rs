use futures::Future;

use etcdv3client::SimpleKvClient;

fn main() {
    env_logger::init();

    let key = "hello";
    let prefix = "hello";
    let value = "world";

    let host: &str = "127.0.0.1";
    let port: u16 = 2379;

    let run = SimpleKvClient::new(host, port)
        .and_then(move |mut client| client.get_string(key.clone()).map(|resp| (client, resp)))
        .and_then(move |(mut client, resp)| {
            println!("resp=> {:?}", resp);

            client.get_with_prefix(prefix)
        })
        .and_then(|kvs| {
            for kv in kvs {
                println!("{} => {:?}", kv.key, String::from_utf8_lossy(&kv.value))
            }
            Ok(())
        })
        .map_err(|e| println!("ERR = {:?}", e));

    tokio::run(run);
}
