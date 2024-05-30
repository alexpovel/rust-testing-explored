---
title: Testing in Rust
subtitle: Towards fearless development
author: 'Alex Povel'
date: '2024-06-05'
theme: dracula
---

# You will get

- an overview of:
  - basics of testing in Rust
  - more advanced approaches & tools
- ideas on improving trust in your systems:
  - uncovering bugs the type system can't
  - keeping documentation in sync
  - understanding it more deeply

# `$ whoami`

- started Rust late 2022
- implemented `srgn` & `b4s` for practice
- today, also use Rust at Cloudflare

# Exploring testing

- Why?
- When?
- How?
- Demo 💥

<!-- ## Basics

- Unit tests
- Integration tests
- Doc tests (+ `README.md`) -->

## Topics

- Doc tests (+ `README.md`)
- Fuzzing
- Property testing
- Snapshot testing
- CI setup (with code coverage)

<!-- # Unit tests

- your testing baseline

## When?

- whenever possible
- but don't bend over backwards

## How?

- Rust has built-in support
- third-party crates for more

## Demo

🧑‍💻

# Integration tests

- tying your units together
- testing behavior of joint systems

## When?

- whenever possible
- often great return on investment

## How?

- Rust has built-in support
- test your public API

## Demo

🧑‍💻 -->

# Doc tests

- your users' first, often only point of contact
- these **determine your interface**

## When?

- whenever possible
- great even for internals
- DTDD?

## How?

- Rust has built-in support
- has some issues

## Demo

🧑‍💻

# Doc tests for `README.md`

- Rust doc tests are specific to Rust code
  - not applicable to binary artifacts
  - do not extend to `README.md`

## When?

- you have a binary crate, aka a user-facing program

## How?

- write custom code ([link](https://github.com/alexpovel/srgn/blob/1a8b3a0bd2f3bb57cc2ede7463ac725a1bb581e4/tests/readme.rs)):
  - minimal `bash` interpreter written with `nom`
  - exercises code snippets in `README.md`
  - uses actual program binary

- hack for libraries: include in source code ([link](https://github.com/alexpovel/b4s/blob/c6ccf71cccfde2e12e1e9e1cc0e07ce5ccf802f2/src/lib.rs#L12)):

  ```rust
  #![doc = include_str!("../README.md")]
  ```

# Fuzzing

- world out there is
  - 👽 scary
  - 🤡 arbitrary
- cannot think of every scenario

## When?

- untrusted, arbitrary user input
- avoid crashing
  - handle any input gracefully

    ```rust
    let s = String::from_utf8(user_input).unwrap();
    ```

  - cannot compare results

    ```rust
    assert_eq!(result, expected);
    //                 ^^^^^^^^
    //                    🤷
    ```

## How?

- source of random bytes
- mold into valid structure
- observe

## Demo

🧑‍💻

# Property

- contracts: ensure property holds
  - testing all `sort(input)` permutations is _impossible_
  - asserting that for any `input`, results are sorted is _trivial_
- more abstract than regular tests, more guided than fuzzing

## When?

- your system has observable properties which should hold
- cannot unit test the world

## How?

- `proptest` ([link](https://docs.rs/proptest/latest/proptest/))
  - based on Python's _Hypothesis_, Haskell's _QuickCheck_

## Demo

🧑‍💻

# Snapshot

- not a technique in itself
- for unit and integration tests on complex data
- fast **reference data creation and review**

## When?

- you have complex, nested or large data structure to craft
- which are perhaps additionally subject to change

## How?

- `insta` ([link](https://docs.rs/insta/latest/insta/))

## Demo

🧑‍💻

# Continuous Integration

- automate running all of these
- make it a frictionless habit
- shared, public understanding
  - slow and flaky parts
  - test coverage

## When?

- whenever possible

## How?

- provider of your choice
  - if your provider makes `cargo test` hard, change providers
- keeping it simple helps CI stay fast and robust
  - C dependencies make cross OS testing harder

## Demo

🧑‍💻

# mutants?
