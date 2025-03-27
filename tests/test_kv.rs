use etcdv3client;

#[tokio::test]
async fn test_kv() {
    let endpoint = "http://localhost:2379";
    let cred = None;
    let mut client = etcdv3client::EtcdClient::new(vec![endpoint], cred)
        .await
        .unwrap();

    let key = "/hello";

    let ret = client.get(key).await;
    assert!(ret.unwrap_err().is_key_not_found());

    let ret = client.put(key, b"world").await;
    assert!(ret.is_ok());

    let ret = client.get(key).await;
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), b"world");

    let ret = client.delete(key).await;
    assert!(ret.is_ok());

    let ret = client.get(key).await;
    assert!(ret.is_err());
    assert!(ret.unwrap_err().is_key_not_found());

    client.put("/test/1", "1").await.unwrap();
    client.put("/test/2", "2").await.unwrap();

    let ret = client.get_with_prefix("/test/").await;
    assert!(ret.is_ok());
    let values = ret
        .unwrap()
        .into_iter()
        .map(|kv| kv.value)
        .collect::<Vec<_>>();
    assert_eq!(values, vec![b"1", b"2"]);

    let ret = client.all().await;
    assert!(ret.is_ok());
    let values = ret
        .unwrap()
        .into_iter()
        .map(|kv| kv.value)
        .collect::<Vec<_>>();
    assert_eq!(values, vec![b"1", b"2"]);
}
