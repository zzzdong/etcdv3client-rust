use etcdv3client;

#[tokio::test]
async fn test_auth() {
    let endpoint = "http://localhost:2379";
    let cred = ("root".to_string(), "123456".to_string());

    let mut client = etcdv3client::EtcdClient::new(vec![endpoint], cred)
        .await
        .unwrap();

    let key = "/hello";
    let ret = client.put(key, b"world").await;
    assert!(ret.is_ok());

    let ret = client.get(key).await;
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), b"world");
}
