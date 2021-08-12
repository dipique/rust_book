pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // error checking should have already happened; this just
        // indicates that there is a bug in that error checking
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100", value);
        }

        Guess { value }
    }

    // this is considered a getter
    pub fn value(&self) -> i32  {
        self.value
    }
}