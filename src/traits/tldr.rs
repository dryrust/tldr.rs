// This is free and unencumbered software released into the public domain.

use crate::{TldrContext, TldrResult};
use alloc::string::String;

/// ```rust
/// # use tldr_traits::{Tldr, TldrContext, TldrResult};
/// # use core::error::Error;
/// struct Rectangle {
///     width: u32,
///     height: u32,
/// }
///
/// impl Tldr<String> for Rectangle {
///     type Error = Box<dyn Error>;
///
///     fn what(&self, _ctx: &TldrContext) -> TldrResult<String> {
///         Ok(Some(format!("A rectangle with a width of {} and a height of {}.", self.width, self.height)))
///     }
/// }
/// ```
///
/// See: https://en.wikipedia.org/wiki/Five_Ws
/// See: https://en.wikipedia.org/wiki/Interrogative_word
pub trait Tldr<T = String> {
    /// The associated error type.
    ///
    /// If in doubt, specify this as `Box<dyn Error>`.
    type Error;

    /// See: https://en.wiktionary.org/wiki/who
    fn who(&self, _ctx: &TldrContext) -> TldrResult<T, Self::Error> {
        Ok(None)
    }

    /// See: https://en.wiktionary.org/wiki/what
    fn what(&self, _ctx: &TldrContext) -> TldrResult<T, Self::Error> {
        Ok(None)
    }

    /// See: https://en.wiktionary.org/wiki/when
    fn when(&self, _ctx: &TldrContext) -> TldrResult<T, Self::Error> {
        Ok(None)
    }

    /// See: https://en.wiktionary.org/wiki/where
    fn r#where(&self, _ctx: &TldrContext) -> TldrResult<T, Self::Error> {
        Ok(None)
    }

    /// See: https://en.wiktionary.org/wiki/why
    fn why(&self, _ctx: &TldrContext) -> TldrResult<T, Self::Error> {
        Ok(None)
    }

    /// See: https://en.wiktionary.org/wiki/whence
    fn whence(&self, _ctx: &TldrContext) -> TldrResult<T, Self::Error> {
        Ok(None)
    }

    /// See: https://en.wiktionary.org/wiki/how
    fn how(&self, _ctx: &TldrContext) -> TldrResult<T, Self::Error> {
        Ok(None)
    }
}
