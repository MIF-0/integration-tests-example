extern crate core;

mod assync_hello_test_without_annotation;
mod sync_test_with_annotation;
mod sync_test_without_annotation;
mod async_hello_test_with_annotation;
mod logger;

use std::future::Future;
use std::pin::Pin;
use std::{panic, thread};
use std::panic::AssertUnwindSafe;
use futures::{executor, FutureExt};
use server_app::Application;
use crate::logger::{log_test, log_static_info, log_error_test};

type AsyncFn = Box<dyn Fn() -> Pin<Box<dyn Future<Output=()>>>>;

async fn setup() {
    log_static_info(format_args!("Starting environment"));
    thread::spawn(move || {
        actix_web::rt::System::new().block_on(async move {
            let app = Application::new("127.0.0.1:9090");
            app.run().await.unwrap();
        })
    });
    log_static_info(format_args!("Setup of environment Finished"));
}

async fn teardown() {
    log_static_info(format_args!("Teardown started"));
    log_static_info(format_args!("Here You can stop APP, db or any other services"));
    log_static_info(format_args!("Teardown finished"));
}

fn main() {
    let init_future = async_main();
    executor::block_on(init_future);
    log_static_info(format_args!("Integration Test Finished"));
}

async fn async_main() {
    let number_of_tests = inventory::iter::<IntegrationTest>.into_iter().count();
    log_static_info(format_args!("Found {} tests", number_of_tests));

    setup().await;
    let system = actix_web::rt::System::new();
    let mut successful_tests = 0;
    let mut failed_tests = 0;
    for t in inventory::iter::<IntegrationTest> {
        if t.test_sync_fn.is_some() {
            log_test(format_args!("Running sync Test: [{}]", t.name));
            let result = panic::catch_unwind(t.test_sync_fn.unwrap());
            match result {
                Ok(_) => {
                    successful_tests = successful_tests + 1;
                    log_test(format_args!("Test [{}] finished with SUCCESS.", t.name))
                }
                Err(e) => {
                    failed_tests = failed_tests + 1;
                    let error: Box<&'static str> = e.downcast().unwrap();
                    log_error_test(format_args!("Test [{}] finished with ERROR. \n {:?}", t.name, error));
                }
            }
        }
        if t.test_async_fn.is_some() {
            log_test(format_args!("Running async Test: [{}]", t.name));
            let async_test = (t.test_async_fn.as_ref().unwrap())();
            let catch_panic_wrapper = AssertUnwindSafe(async_test).catch_unwind();
            let result = system.block_on(catch_panic_wrapper);
            match result {
                Ok(_) => {
                    successful_tests = successful_tests + 1;
                    log_test(format_args!("Test [{}] finished with SUCCESS.", t.name))
                }
                Err(e) => {
                    failed_tests = failed_tests + 1;
                    let error: Box<&'static str> = e.downcast().unwrap();
                    log_error_test(format_args!("Test [{}] finished with ERROR. \n {:?}", t.name, error));
                }
            }
        }
    }

    // Teardown test environment
    teardown().await;
    log_static_info(format_args!("{} FAILED tests. {} SUCCESSful tests", failed_tests, successful_tests));
    if failed_tests > 0 {
        panic!("Integration tests are failed")
    }
}


struct IntegrationTest {
    pub name: &'static str,
    pub test_sync_fn: Option<fn()>,
    pub test_async_fn: Option<AsyncFn>,
}

impl IntegrationTest {

    fn async_test(name: &'static str, function: AsyncFn) -> IntegrationTest {
        IntegrationTest {
            name,
            test_sync_fn: None,
            test_async_fn: Some(function),
        }
    }

    pub fn sync_test(name: &'static str, function: fn()) -> IntegrationTest {
        IntegrationTest {
            name,
            test_sync_fn: Some(function),
            test_async_fn: None,
        }
    }
}

inventory::collect!(IntegrationTest);