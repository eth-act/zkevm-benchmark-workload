//! Benchmark runner library for zkVM benchmarking

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

pub mod guest_programs;

pub mod block_encoding_length_program;
pub mod empty_program;
pub mod stateless_validator;

pub mod runner;
