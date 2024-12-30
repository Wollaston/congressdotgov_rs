//! Rust bindings to the congress.gov REST API.
//!
//! # High-level features
//!
//! - REST API bindings are divided between Endpoint, Client, and Query traits
//! - Auth and state are managed by the Cdg struct
//! - Optional Endpoint parameters are added to a query via a Builder API
//! - Responses are returned as a serde_json::Value

pub mod api;
mod auth;
mod cdg;
mod client;
mod endpoint;
mod params;
mod query;

pub use crate::cdg::{Cdg, CdgError};
