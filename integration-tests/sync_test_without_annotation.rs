use super::IntegrationTest;

fn basic_test() {
    assert!(false, "How come?")
}

inventory::submit!(IntegrationTest::sync_test("basic sync test", basic_test));