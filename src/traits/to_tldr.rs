// This is free and unencumbered software released into the public domain.

use crate::Tldr;
use alloc::{boxed::Box, string::String};

pub trait ToTldr<T = String> {
    /// The associated error type.
    ///
    /// If in doubt, specify this as `Box<dyn Error>`.
    type Error;

    fn to_tldr(&self) -> Box<dyn Tldr<T, Error = Self::Error>>;
}
