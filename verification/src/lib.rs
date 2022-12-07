#![no_std]

extern crate alloc;

pub mod constants;
pub mod types;

mod utilities;
pub use utilities::{mmr, ssz, trie};

#[cfg(test)]
mod tests;
