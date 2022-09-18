use std::io; //import the whole shebang?
use std::cmp::Ordering;
use rand::Rng; //import a specific trait.

pub mod double {

    struct doubleLast {
        last: i64,
    }

    impl doubleLast {
        fn step (&mut self, z: i64) -> i64 {
            let thisOne = self.last;
            self.last = z;
            return thisOne*2;
        }
    }


}
