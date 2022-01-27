#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

#[macro_use]
extern crate log;

/// A representation of a range of commits within a Git repository.
mod commits;

pub use crate::commits::Commits;
