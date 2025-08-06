// This is free and unencumbered software released into the public domain.

use crate::TldrLanguage;
use core::str::FromStr;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct TldrContext {
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
