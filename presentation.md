---
title: Testing in Rust explored
subtitle: Towards fearless development
author: 'Alex Povel'
date: '2024-06-05'
theme: dracula
---
<!-- markdownlint-disable MD025 -->

# You will get

- an overview of...
  - basics of testing in Rust
  - more advanced approaches & tools
- ...improving your software:
  - uncovering bugs the type system can't
  - keeping documentation in sync
  - understanding it more deeply

# `$ whoami`

- started Rust late 2022
- implemented `srgn` for practice
- today, use Rust at Cloudflare as well

# Exploring testing

## Basics

- unit tests
- integration tests
- doc tests (+ `README.md`)

## Advanced

- fuzzing
- snapshot tests
- property tests
- benchmarks

# Code samples

## Unit tests

## Integration tests

## Doc tests

- do not extend to `README.md`. Solutions:
  - [include in source code](https://github.com/alexpovel/b4s/blob/c6ccf71cccfde2e12e1e9e1cc0e07ce5ccf802f2/src/lib.rs#L12):

    ```rust
    #![doc = include_str!("../README.md")]
    ```

  - [write custom code](https://github.com/alexpovel/srgn/blob/1a8b3a0bd2f3bb57cc2ede7463ac725a1bb581e4/tests/readme.rs)
