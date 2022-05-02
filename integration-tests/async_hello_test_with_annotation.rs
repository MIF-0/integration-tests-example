use log::info;
use super::IntegrationTest;

//Annotation is not needed, but can be useful to run test via IDE

#[actix_web::test]
async fn hello_test() {
    hello_test_logic.await;
}

pub async fn hello_test_logic() {
    let client = reqwest::Client::builder()
        .build()
        .expect("error during client build");
    let response = client.get("http://localhost:9091/").send().await;
    info!("{:?}", response);
    assert!(response.is_ok());
}

inventory::submit!(IntegrationTest::async_test ("Call remote server test: success", Box::new(|| Box::pin(hello_test_logic()))));