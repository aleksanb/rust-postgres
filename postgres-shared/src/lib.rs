#![allow(unknown_lints)] // for clippy

extern crate hex;
extern crate fallible_iterator;
extern crate phf;
extern crate postgres_protocol;

pub mod error;
pub mod params;
pub mod types;
pub mod rows;
pub mod stmt;

/// Contains information necessary to cancel queries for a session.
#[derive(Copy, Clone, Debug)]
pub struct CancelData {
    /// The process ID of the session.
    pub process_id: i32,
    /// The secret key for the session.
    pub secret_key: i32,
}
