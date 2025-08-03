// This is free and unencumbered software released into the public domain.

//! This crate provides abstractions for TL;DR summarization.

#![no_std]
#![forbid(unsafe_code)]

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

extern crate alloc;
