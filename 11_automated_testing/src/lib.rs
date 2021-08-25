#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height >= other.height && self.width >= other.width
    }
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("That value is some 0 or negative bullshit: {}", value);
        } else if value > 100 {
            panic!("That big ass number '{}' does not belong here", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_does_not_work() {
        assert_eq!(2+2, 5);
    }

    #[test]
    fn panic_to_fail() {
        panic!("oh my god what do I do");
    }

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 4,
            height: 5,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 4,
            height: 5,
        };
        assert!(!smaller.can_hold(&larger));
    }

    // custom error messages
    #[test]
    fn greeting_contains_name() {
        let result = String::from("carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // use should_panic attribute to indicate that the result
    // of the test should be a panic
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(101);
    }

    // expect a specific panic string
    #[test]
    #[should_panic(expected = "That value is some 0 or negative bullshit: 0")]
    fn less_than_1() {
        Guess::new(0);
    }

    // failing with Err from a result
    #[test]
    fn result_test() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("two plus two does not equal 4".into())
        }
    }
}

// to set the # of test threads (by default it is run in parallel):
//      cargo test -- --test-threads=1
// not sure what the extra double dash does but it is required