use std::fs;

pub(crate) fn day01() {
    let input = fs::read_to_string("input/day01/test.txt").unwrap();
}

#[cfg(test)]
mod day01_tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
