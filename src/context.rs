// This is free and unencumbered software released into the public domain.

use crate::TldrLanguage;
use core::str::FromStr;

/// A context used when generating TL;DR summaries.
///
/// # Examples
///
/// ```rust
/// # use tldr_traits::TldrContext;
/// let context = TldrContext::builder()
///     .language("en")
///     .build();
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
#[cfg_attr(feature = "builder", builder(on(TldrLanguage, into)))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct TldrContext {
    #[cfg_attr(feature = "builder", builder(default))]
    pub language: TldrLanguage,
}

impl FromStr for TldrContext {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(TldrContext {
            language: input.parse()?,
        })
    }
}
