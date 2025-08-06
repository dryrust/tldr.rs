// This is free and unencumbered software released into the public domain.

use alloc::string::String;
use core::{fmt, str::FromStr};

#[non_exhaustive]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum TldrLanguage {
    #[default]
    English,

    Other(String),
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
