use log::info;
use super::IntegrationTest;

async fn hello_test_err() {
    let client = reqwest::Client::builder()
        .build()
        .expect("error during client build");
    let response = client.get("http://localhost:9090/").send().await;
    info!("{:?}", response);
    assert!(response.is_ok());
}

inventory::submit!(IntegrationTest::async_test ("Call remote server test: checking error", Box::new(|| Box::pin(hello_test_err()))));