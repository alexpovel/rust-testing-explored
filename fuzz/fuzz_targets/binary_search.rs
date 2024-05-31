#![no_main]

use libfuzzer_sys::fuzz_target;
use rust_testing_explored::fuzzing::{binary_search, AsciiChar};

fuzz_target!(|input: (String, String, u8)| {
    let needle = input.0;
    let haystack = input.1;
    if let Ok(sep) = AsciiChar::from_ascii(input.2) {
        // 1. Just assert it doesn't panic
        let _ = binary_search(needle.as_str(), haystack.as_str(), sep);

        // 2. Assert something more interesting, such as 'anything found cannot be beyond
        //    the bounds of the input'
        match binary_search(needle.as_str(), haystack.as_str(), sep) {
            // `usize` always positive, no need to test `start >= 0`
            Ok(range) | Err(range) => assert!(range.end <= haystack.len()),
        };
    };
});
