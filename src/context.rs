// This is free and unencumbered software released into the public domain.

use alloc::string::String;
use core::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TldrContext {
    pub language: String,
}

impl FromStr for TldrContext {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(TldrContext {
            language: input.into(),
        })
    }
}
