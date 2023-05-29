#![no_std]

extern crate alloc;

#[cfg(test)]
extern crate std;

#[macro_use]
mod log;

pub mod consensus_specs;
pub mod error;
pub mod types;

mod utilities;
pub use utilities::{mmr, ssz, trie};

#[cfg(test)]
mod tests;

pub extern crate molecule;
