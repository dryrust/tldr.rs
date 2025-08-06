// This is free and unencumbered software released into the public domain.

use crate::Tldr;
use alloc::{boxed::Box, string::String};

/// ```rust
/// # use tldr_traits::{Tldr, TldrSummary, ToTldr};
/// struct Rectangle {
///     width: u32,
///     height: u32,
/// }
///
/// impl ToTldr<String> for Rectangle {
///     type Error = Box<dyn core::error::Error>;
///
///     fn to_tldr(&self) -> Box<dyn Tldr<String, Error = Self::Error>> {
///         todo!() // FIXME
///     }
/// }
/// ```
pub trait ToTldr<T = String> {
    /// The associated error type.
    ///
    /// If in doubt, specify this as `Box<dyn Error>`.
    type Error;

    fn to_tldr(&self) -> Box<dyn Tldr<T, Error = Self::Error>>;
}
