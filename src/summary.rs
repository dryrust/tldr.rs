// This is free and unencumbered software released into the public domain.

use crate::{Tldr, TldrContext, ToTldr};
use alloc::{boxed::Box, string::String};

/// See: https://en.wikipedia.org/wiki/Five_Ws
/// See: https://en.wikipedia.org/wiki/Interrogative_word
#[derive(Clone, Debug)]
pub struct TldrSummary {
    /// See: https://en.wiktionary.org/wiki/who
    pub who: Option<String>,

    /// See: https://en.wiktionary.org/wiki/what
    pub what: Option<String>,

    /// See: https://en.wiktionary.org/wiki/when
    pub when: Option<String>,

    /// See: https://en.wiktionary.org/wiki/where
    pub r#where: Option<String>,

    /// See: https://en.wiktionary.org/wiki/why
    pub why: Option<String>,

    /// See: https://en.wiktionary.org/wiki/whence
    pub whence: Option<String>,

    /// See: https://en.wiktionary.org/wiki/how
    pub how: Option<String>,
}

impl ToTldr for TldrSummary {
    fn to_tldr(&self) -> Box<dyn Tldr> {
        Box::new(self.clone())
    }
}

impl Tldr for TldrSummary {
    fn who(&self, _ctx: &TldrContext) -> Option<String> {
        self.who.clone()
    }

    fn what(&self, _ctx: &TldrContext) -> Option<String> {
        self.what.clone()
    }

    fn when(&self, _ctx: &TldrContext) -> Option<String> {
        self.when.clone()
    }

    fn r#where(&self, _ctx: &TldrContext) -> Option<String> {
        self.r#where.clone()
    }

    fn why(&self, _ctx: &TldrContext) -> Option<String> {
        self.why.clone()
    }

    fn whence(&self, _ctx: &TldrContext) -> Option<String> {
        self.whence.clone()
    }

    fn how(&self, _ctx: &TldrContext) -> Option<String> {
        self.how.clone()
    }
}
