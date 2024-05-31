//! Copied from https://github.com/alexpovel/b4s/tree/c6ccf71cccfde2e12e1e9e1cc0e07ce5ccf802f2
pub use ascii::AsciiChar;
use itertools::Itertools;
use std::{cmp::Ordering, ops::Range};

pub fn binary_search(
    needle: &str,
    haystack: &str,
    sep: AsciiChar,
) -> Result<Range<usize>, Range<usize>> {
    let leftmost = 0;
    let rightmost = haystack.len();

    let mut low = leftmost;
    let mut high = rightmost;

    let mut start = leftmost;
    let mut end = rightmost;

    let haystack = haystack.as_bytes();

    let pred = |c: &&u8| **c == sep.as_byte();

    while low < high {
        let mid = low + (high - low) / 2;

        start = match haystack[..mid].iter().rev().find_position(pred) {
            Some((delta, _)) => mid - delta,
            None => leftmost,
        };

        end = match haystack[mid..].iter().find_position(pred) {
            Some((delta, _)) => mid + delta,
            None => rightmost,
        };

        let range = if cfg!(fuzzing) {
            start..(end + 1) // ⚠️ Off-by-one error
        } else {
            start..end
        };

        let haystack_word = std::str::from_utf8(&haystack[range])
            .expect("Indices aren't valid for slicing into haystack. They are at ASCII chars and therefore always assumed valid.");

        match needle.cmp(haystack_word) {
            Ordering::Less => high = mid.saturating_sub(1),
            Ordering::Equal => return Ok(Range { start, end }),
            Ordering::Greater => low = mid + 1,
        }
    }

    Err(Range { start, end })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary_search() {
        let needle = "x";
        let haystack = r#"a
b
cdef
Hello World
k
l
m
What a beautiful day 😀
x
zzzzzzzz
"#;
        let sep = AsciiChar::LineFeed;

        let expected_at = 53..54;
        assert_eq!(
            binary_search(needle, haystack, sep),
            Ok(expected_at.clone())
        );
        assert_eq!(&haystack[expected_at], needle);
    }
}
