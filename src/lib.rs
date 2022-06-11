#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
#![allow(clippy::tabs_in_doc_comments)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod object;
mod pool;
mod reset;

pub use crate::{object::Pooled, pool::Pool, reset::Reset};
