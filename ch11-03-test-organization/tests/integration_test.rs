use ch11_03_test_organization;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    common::sub_module::setup2();
    assert_eq!(4, ch11_03_test_organization::add_two(2));
}
