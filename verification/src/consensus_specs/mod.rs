//! Constants from [Ethereum Proof-of-Stake Consensus Specifications]
//!
//! [Ethereum Proof-of-Stake Consensus Specifications]: https://github.com/ethereum/consensus-specs

#[doc(hidden)]
#[macro_use]
pub mod macros;

pub mod forks;
pub mod helpers;

mod internal;
pub use internal::*;
