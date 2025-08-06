// This is free and unencumbered software released into the public domain.

use crate::{Tldr, TldrContext, TldrResult, ToTldr};
use alloc::{boxed::Box, string::String};
use core::fmt::Debug;

/// ```rust
/// # use tldr_traits::TldrSummary;
/// let summary: TldrSummary<String> = TldrSummary::builder()
///     .who("John")
///     .what("a book")
///     .when("yesterday")
///     .r#where("the library")
///     .why("to learn")
///     .whence("from the internet")
///     .how("by reading")
///     .build();
/// ```
///
/// See: https://en.wikipedia.org/wiki/Five_Ws
/// See: https://en.wikipedia.org/wiki/Interrogative_word
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
#[cfg_attr(feature = "builder", builder(on(T, into)))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct TldrSummary<T: Clone + Debug + Default = String> {
    /// See: https://en.wiktionary.org/wiki/who
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub who: Option<T>,

    /// See: https://en.wiktionary.org/wiki/what
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub what: Option<T>,

    /// See: https://en.wiktionary.org/wiki/when
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub when: Option<T>,

    /// See: https://en.wiktionary.org/wiki/where
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub r#where: Option<T>,

    /// See: https://en.wiktionary.org/wiki/why
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub why: Option<T>,

    /// See: https://en.wiktionary.org/wiki/whence
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub whence: Option<T>,

    /// See: https://en.wiktionary.org/wiki/how
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub how: Option<T>,
}

impl<T: Clone + Debug + Default + 'static> ToTldr<T> for TldrSummary<T> {
    type Error = ();

    fn to_tldr(&self) -> Box<dyn Tldr<T, Error = Self::Error>> {
        Box::new(self.clone())
    }
}

impl<T: Clone + Debug + Default> Tldr<T> for TldrSummary<T> {
    type Error = ();

    fn who(&self, _ctx: &TldrContext) -> TldrResult<T, Self::Error> {
        Ok(self.who.clone())
    }

    fn what(&self, _ctx: &TldrContext) -> TldrResult<T, Self::Error> {
        Ok(self.what.clone())
    }

    fn when(&self, _ctx: &TldrContext) -> TldrResult<T, Self::Error> {
        Ok(self.when.clone())
    }

    fn r#where(&self, _ctx: &TldrContext) -> TldrResult<T, Self::Error> {
        Ok(self.r#where.clone())
    }

    fn why(&self, _ctx: &TldrContext) -> TldrResult<T, Self::Error> {
        Ok(self.why.clone())
    }

    fn whence(&self, _ctx: &TldrContext) -> TldrResult<T, Self::Error> {
        Ok(self.whence.clone())
    }

    fn how(&self, _ctx: &TldrContext) -> TldrResult<T, Self::Error> {
        Ok(self.how.clone())
    }
}
