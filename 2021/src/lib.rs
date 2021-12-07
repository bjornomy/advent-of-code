pub mod puzzle {

    pub fn split_to_tuple<'inp>(input: &'inp str, pat: &str) -> (&'inp str, &'inp str) {
        let splits :Vec<&str> = input.split(pat).collect();
        (splits[0], splits[1])
    }

    pub trait Puzzle {

        fn solve(input: &str);

    }
}
