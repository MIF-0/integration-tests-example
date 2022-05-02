use super::IntegrationTest;

//Annotation is not needed, but can be useful to run test via IDE

#[test]
fn basic_test() {
    basic_test_logic();
}

fn basic_test_logic() {
    assert!(true, "How come?")
}

inventory::submit!(IntegrationTest::sync_test("basic sync test with annotation", basic_test_logic));

