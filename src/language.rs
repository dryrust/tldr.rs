// This is free and unencumbered software released into the public domain.

use alloc::string::String;
use core::{fmt, str::FromStr};

/// The language that a TL;DR summary is written in.
///
/// # Examples
///
/// ```rust
/// # use tldr_traits::TldrLanguage;
/// let language = TldrLanguage::default(); // this is the same as...
/// let language = TldrLanguage::English;
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TldrLanguage {
    #[default]
    #[cfg_attr(feature = "serde", serde(rename = "en"))]
    English,

    Other(String),
}

impl From<&str> for TldrLanguage {
    fn from(input: &str) -> Self {
        match input {
            "en" => TldrLanguage::English,
            _ => TldrLanguage::Other(input.into()),
        }
    }
}

impl FromStr for TldrLanguage {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "en" => TldrLanguage::English,
            _ => TldrLanguage::Other(input.into()),
        })
    }
}

impl fmt::Display for TldrLanguage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TldrLanguage::English => write!(f, "en"),
            TldrLanguage::Other(lang) => write!(f, "{}", lang),
        }
    }
}
