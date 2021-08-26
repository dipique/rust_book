// to create integration tests, which are run externally
// from the library:
//   1) create a tests folder in root dir (next to src)
//   2) create an rs file in tests, like this one
//   3) write tests (couldn't actually get them to run as
//      expected without specifying, i.e. using command
//          cargo test --test integration_tests)

use automated_testing;
mod common; // reference testing helper file

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, automated_testing::add_two(2));
}

// to run tests from this file:
//      cargo test --test integration_tests