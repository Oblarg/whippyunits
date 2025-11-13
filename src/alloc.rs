//! Compatibility module for alloc/std types
//!
//! This module provides a unified interface for types that differ between
//! `std` and `alloc` environments, allowing the rest of the codebase to use
//! `crate::alloc::*` without feature flag duplication.

#[cfg(not(feature = "std"))]
pub use alloc_crate::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

#[cfg(feature = "std")]
pub use std::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
