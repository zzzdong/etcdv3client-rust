use etcdv3client::{EtcdClientError, SimpleAuthClient, SimpleKvClient};

#[tokio::main]
async fn main() -> Result<(), EtcdClientError> {
    env_logger::init();

    let key = "hello";
    let prefix = "hello";
    let value = "world";

    let endpoint = "http://localhost:2379";

    let name = "root";
    let password = "123456";

    let token: Option<String> = None;

    let mut client = SimpleAuthClient::new(vec![endpoint], token).await?;

    let token = client.get_token(name, password).await?;
    println!("token=> {:?}", token);

    let mut client = SimpleKvClient::new(vec![endpoint], Some(token)).await?;

    match client.get_string(key.clone()).await {
        Ok(resp) => {
            println!("get `{}`: {}", key, resp);
        }
        Err(EtcdClientError::KeyNotFound) => {
            client.put(key.clone(), value.clone()).await?;
        }
        Err(_) => {
            return Err(EtcdClientError::ErrMsg("get_string failed".to_string()));
        }
    }

    let kvs = client.get_with_prefix(prefix).await?;

    for kv in kvs {
        println!(
            "{} => {:?}",
            String::from_utf8_lossy(&kv.key),
            String::from_utf8_lossy(&kv.value)
        )
    }

    Ok(())
}
