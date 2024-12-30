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
//! use congressdotgov_rs::api::{self, Query};
//! use congressdotgov_rs::api::bill::Congress;
//!
//! // The return type of a `Bill`. Note that Cdg may contain more information, but you can
//! // define your structure to only fetch what is needed.
//! #[derive(Debug, Deserialize)]
//! struct Bill {
//!     title: String,
//! }
//!
//! // Create the client.
//! let auth = Auth::Token("API_KEY".into()).unwrap();
//! let client = Cdg::new(auth).unwrap();
//!
//! // Create a simple endpoint. This one gets recent Bills from the 118th Congress.
//! let endpoint = Congress::builder().congress(118_u8).build().unwrap();
//! // Call the endpoint. The return type decides how to represent the value.
//! let bills: Vec<Bill> = endpoint.query(&client).unwrap();
//! ```

use std::borrow::Cow;

pub mod amendments;
pub mod bill;
pub mod bound_congressional_record;
pub mod committee;
pub mod committee_meeting;
pub mod committee_print;
pub mod committee_report;
pub mod congressional_record;
pub mod daily_congressional_record;
pub mod error;
pub mod hearing;
pub mod house_communication;
pub mod house_requirement;
pub mod law;
pub mod member;
pub mod nomination;
pub mod senate_communication;
pub mod summaries;
pub mod treaty;

/// Chamber options for Committee endpoints.
///
/// This differs from the Chamber enum for the Committee
/// resource as this has the NoChamber variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitteeChamber {
    House,
    Senate,
    NoChamber,
}

impl CommitteeChamber {
    fn as_str(self) -> &'static str {
        match self {
            CommitteeChamber::House => "house",
            CommitteeChamber::Senate => "senate",
            CommitteeChamber::NoChamber => "nochamber",
        }
    }
}

/// The possible Congressional bill types for both
/// the House of Representatives and Senate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillType {
    /// H.R. - House Bill
    Hr,
    /// S. - Senate Bill
    S,
    /// H.J. Res. - House Joint Resolution
    Hjres,
    /// S.J. Res. - Senate Joint Resolution
    Sjres,
    /// S.J. Res. - House Concurrent Resolution
    Hconres,
    /// S. Con. Res. - Senate Concurrent Resolution
    Sconres,
    /// H. Res. - House Simple Resolution
    Hres,
    /// S. Res - Senate Simple Resolution
    Sres,
}

impl BillType {
    fn as_str(self) -> &'static str {
        match self {
            BillType::Hr => "hr",
            BillType::S => "s",
            BillType::Hjres => "hjres",
            BillType::Sjres => "sjres",
            BillType::Hconres => "hconres",
            BillType::Sconres => "sconres",
            BillType::Hres => "hres",
            BillType::Sres => "sres",
        }
    }
}

/// The congress.gov API can return data in either Json or XML
/// format. The default for this crate is Json.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Json,
    Xml,
}

impl Default for Format {
    fn default() -> Self {
        Self::Json
    }
}

impl From<Format> for Cow<'static, str> {
    fn from(format: Format) -> Self {
        format.as_str().into()
    }
}

impl Format {
    pub fn as_str(self) -> &'static str {
        match self {
            Format::Json => "json",
            Format::Xml => "xml",
        }
    }
}

/// Certain endpoints allow the response to be sorted
/// in either Ascending or Descending order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sort {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

impl Default for Sort {
    fn default() -> Self {
        Self::Asc
    }
}

impl Sort {
    pub fn as_str(self) -> &'static str {
        match self {
            Sort::Asc => "asc",
            Sort::Desc => "desc",
        }
    }
}
