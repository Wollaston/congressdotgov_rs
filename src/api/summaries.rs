#![allow(clippy::module_inception)]

//! Summaries API endpoints.

mod bill_type;
mod congress;
mod summaries;

pub use self::bill_type::{BillType, BillTypeBuilder, BillTypeBuilderError};
pub use self::congress::{Congress, CongressBuilder, CongressBuilderError};
pub use self::summaries::{Summaries, SummariesBuilder, SummariesBuilderError};
