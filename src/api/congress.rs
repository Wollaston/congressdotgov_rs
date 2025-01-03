#![allow(clippy::module_inception)]

//! Congress API endpoints and types.

mod congress;
mod current_congress;
mod specific_congress;

pub use self::congress::{Congress, CongressBuilder, CongressBuilderError};
pub use self::current_congress::{
    CurrentCongress, CurrentCongressBuilder, CurrentCongressBuilderError,
};
pub use self::specific_congress::{
    SpecificCongress, SpecificCongressBuilder, SpecificCongressBuilderError,
};
