//! # raur-ext
//!
//! raur-ext is an extension to the raur AUR RPC library. It provides request splitting
//! (requests are limited to URI of 4443 bytes maximum length). And a way to cache
//! packages into a hash set and query them by pkgname.

#![warn(missing_docs)]
mod cache;
mod raur;

pub use crate::cache::*;
pub use crate::raur::*;
