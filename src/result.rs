// This is free and unencumbered software released into the public domain.

use alloc::{boxed::Box, string::String};
use core::error::Error;

pub type TldrResult<T = String, E = Box<dyn Error>> = Result<Option<T>, E>;
