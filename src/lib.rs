// This is free and unencumbered software released into the public domain.

//! This crate provides abstractions for TL;DR summarization using the [five Ws]:
//!
//! > **The Five Ws** is a checklist used in journalism to ensure that the lead
//! > contains all the essential points of a story. As far back as 1913,
//! > reporters were taught that the lead should answer these questions:
//! >
//! > - Who? – asking about a person or other agent
//! > - What? – asking about an object or action
//! > - When? – asking about a time
//! > - Where? – asking about a place
//! > - Why? – asking about a reason or cause
//! >
//! > In modern times, journalism students are still taught that these are the
//! > fundamental five questions of newswriting.
//!
//! [five Ws]: https://en.wikipedia.org/wiki/Five_Ws
//!
//! # Examples
//!
//! ```rust,ignore
//! use tldr::{Tldr, TldrContext, TldrLanguage, TldrResult, TldrSummary, ToTldr};
//! ```
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
