//! API endpoint structures
//!
//! The types in this module are meant to aid in constructing the appropriate calls using type-safe
//! Rust idioms.
//!
//! All endpoints use the builder pattern and have their members as private so that there are no
//! API implications of adding new members for additional query parameters in future releases.
//!
//! # Example
//!
//! ```rust,no_run
//! use serde::Deserialize;
//! use congressdotgov_rs::Cdg;
//! use congressdotgov_rs::api::Query;
//! use congressdotgov_rs::api::bill;
//! use congressdotgov_rs::Auth;
//! use tokio_test::block_on;
//!
//! // The return type of a `Bill`. Note that a Bill may contain more information, but you can
//! // define your structure to only deserialize what is needed as the return value is a
//! // serde_json::Value.
//! #[derive(Debug, Deserialize)]
//! struct Bill {
//!     title: String,
//! }
//!
//! // Create the client.
//! let auth = Auth::Token("API_KEY".into());
//! let client = Cdg::new(auth).unwrap();
//!
//! // Create a simple endpoint. This one gets recent Bills from the 118th Congress.
//! let endpoint = bill::Congress::builder().congress(118_u8).build().unwrap();
//!
//! // Call the endpoint. The return type decides how to represent the value.
//! # tokio_test::block_on(async {
//! let bills: Vec<Bill> = endpoint.query(&client).await.unwrap();
//! # })
//! ```

mod client;
mod endpoint;
mod error;
mod params;
pub(crate) mod query;

pub mod common;

pub mod amendments;
pub mod bill;
pub mod bound_congressional_record;
pub mod committee;
pub mod committee_meeting;
pub mod committee_print;
pub mod committee_report;
pub mod congress;
pub mod congressional_record;
pub mod daily_congressional_record;
pub mod hearing;
pub mod house_communication;
pub mod house_requirement;
pub mod law;
pub mod member;
pub mod nomination;
pub mod senate_communication;
pub mod summaries;
pub mod treaty;

pub use self::client::Client;

pub use self::endpoint::Endpoint;
pub use self::endpoint::UrlBase;

pub use self::error::ApiError;

pub use self::params::ParamValue;
pub use self::params::QueryParams;

pub use self::query::Query;
