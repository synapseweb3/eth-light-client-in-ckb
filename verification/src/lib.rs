#![no_std]

extern crate alloc;

#[cfg(test)]
extern crate std;

pub mod constants;
pub mod types;

mod utilities;
pub use utilities::{mmr, ssz, trie};

#[cfg(test)]
mod tests;

pub extern crate molecule;
