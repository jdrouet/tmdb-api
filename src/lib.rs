//! Another implementation of a client for the TMDB API
//!
//! It provides a support for async and implements each command using the Command pattern.

#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_repr;

/// The used version of reqwest
#[cfg(feature = "commands")]
pub use reqwest;

pub mod certification;
pub mod changes;
#[cfg(feature = "commands")]
pub mod client;
pub mod collection;
pub mod company;
pub mod error;
pub mod genre;
pub mod movie;
pub mod people;
#[cfg(feature = "commands")]
pub mod prelude;
pub mod tvshow;

pub mod common;
mod util;

#[cfg(feature = "commands")]
pub use client::Client;
