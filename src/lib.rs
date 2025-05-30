#![doc = include_str!("../readme.md")]
#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_repr;

/// The used version of chrono
pub use chrono;
/// The used version of reqwest
pub use reqwest;

pub use client::Client;

pub mod certification;
pub mod changes;

pub mod client;
pub mod collection;
pub mod company;
pub mod error;
pub mod find;
pub mod genre;
pub mod movie;
pub mod people;
pub mod tvshow;
pub mod watch_provider;

pub mod common;
pub mod configuration;
mod util;

pub type Result<V> = std::result::Result<V, crate::error::Error>;
