use std::fs;

pub(crate) fn day24() {
    let input = fs::read_to_string("input/day24/test.txt").unwrap();
}

#[cfg(test)]
mod day24_tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
