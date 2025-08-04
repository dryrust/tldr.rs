// This is free and unencumbered software released into the public domain.

use crate::Tldr;
use alloc::boxed::Box;

pub trait ToTldr {
    fn to_tldr(&self) -> Box<dyn Tldr>;
}
