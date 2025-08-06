// This is free and unencumbered software released into the public domain.

//! This crate provides abstractions for TL;DR summarization using the five Ws.

#![no_std]
#![forbid(unsafe_code)]

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

extern crate alloc;

mod context;
pub use context::*;

mod language;
pub use language::*;

mod result;
pub use result::*;

mod summary;
pub use summary::*;

mod traits;
pub use traits::*;
