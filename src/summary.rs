// This is free and unencumbered software released into the public domain.

use crate::{Tldr, TldrContext, TldrResult, ToTldr};
use alloc::{boxed::Box, string::String};
use core::fmt::Debug;

/// See: https://en.wikipedia.org/wiki/Five_Ws
/// See: https://en.wikipedia.org/wiki/Interrogative_word
#[derive(Clone, Debug, Default)]
pub struct TldrSummary<T: Clone + Debug + Default = String> {
    /// See: https://en.wiktionary.org/wiki/who
    pub who: Option<T>,

    /// See: https://en.wiktionary.org/wiki/what
    pub what: Option<T>,

    /// See: https://en.wiktionary.org/wiki/when
    pub when: Option<T>,

    /// See: https://en.wiktionary.org/wiki/where
    pub r#where: Option<T>,

    /// See: https://en.wiktionary.org/wiki/why
    pub why: Option<T>,

    /// See: https://en.wiktionary.org/wiki/whence
    pub whence: Option<T>,

    /// See: https://en.wiktionary.org/wiki/how
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
