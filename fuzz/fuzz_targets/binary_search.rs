#![no_main]

use libfuzzer_sys::fuzz_target;
// use rust_testing_explored::binary_search;

fuzz_target!(|data: &[u8]| {
    let x = 3;
    // fuzzed code goes here
});
