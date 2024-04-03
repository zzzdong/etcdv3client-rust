use std::time::Duration;

use tokio::time::sleep;

use etcdv3client::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let key = "hello";
    let world = "world";

    let endpoint = "http://localhost:2379";
    let cred = None;

    let mut client = Client::new(vec![endpoint], cred).await?;

    let start = std::time::Instant::now();
    let lease = client.grant_lease(3).await?;

    let leases = client.list_leases().await?;
    println!("got all lease: {:?}", leases);

    client.kv.do_put(key, world).with_lease(lease.id).await?;
    println!("put {}-{} done at {:?}", key, world, start.elapsed());

    sleep(Duration::from_secs(1)).await;
    // after 1 second, lease still available
    let info = client.get_lease_info(lease.id, true).await?;
    println!("got lease: {:?}", info);

    let value = client.get_string(key).await;
    println!("try get {}: {:?} at {:?}", key, value, start.elapsed());

    sleep(Duration::from_secs(3)).await;
    // after 1+3 seconds, lease unavailable
    let value = client.get_string(key).await;
    println!("try get {}: {:?} at {:?}", key, value, start.elapsed());
    assert_eq!(value.is_err(), true);

    // another case with keep_alive
    let start = std::time::Instant::now();
    let lease = client.grant_lease(3).await?;

    client.kv.do_put(key, world).with_lease(lease.id).await?;

    println!("put {} with {:?} at {:?} done", key, world, start.elapsed());

    let mut aliver = client.keep_lease_alive(lease.id).await?;
    loop {
        sleep(Duration::from_secs(1)).await;

        let elapsed = start.elapsed();
        if elapsed < Duration::from_secs(3) {
            aliver.keep_alive().await?;
            print!("after {:?}, aliver is alive\t", elapsed);
            print!(
                "TTL is {:?} \t",
                aliver.message().await.unwrap().unwrap().ttl
            )
        } else {
            print!("after {:?}, aliver is dead\t", elapsed);
        }

        match client.get_string(key).await {
            Ok(_v) => {
                println!("{} still alive", key);
            }
            Err(e) => {
                println!("get {} error with {:?}", key, e);
                break;
            }
        }
    }

    Ok(())
}
