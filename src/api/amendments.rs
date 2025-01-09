#![allow(clippy::module_inception)]

//! Amendments API endpoints and types.

use serde::{Deserialize, Serialize};

mod actions;
mod amendment;
mod amendment_number;
mod amendment_type;
mod amendments;
mod congress;
mod cosponsors;
mod text;

pub use self::actions::{Actions, ActionsBuilder, ActionsBuilderError};
pub use self::amendment::{Amendment, AmendmentBuilder, AmendmentBuilderError};
pub use self::amendment_number::{
    AmendmentNumber, AmendmentNumberBuilder, AmendmentNumberBuilderError,
};
pub use self::amendment_type::{AmendmentType, AmendmentTypeBuilder, AmendmentTypeBuilderError};
pub use self::amendments::{Amendments, AmendmentsBuilder, AmendmentsBuilderError};
pub use self::congress::{Congress, CongressBuilder, CongressBuilderError};
pub use self::cosponsors::{Cosponsors, CosponsorsBuilder, CosponsorsBuilderError};
pub use self::text::{Text, TextBuilder, TextBuilderError};

/// The possible Amendment Types in Congress.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CongressionalAmendmentType {
    /// H.Amdt. - House amendment. Amends a House bill.
    Hamdt,
    /// S.Amdt. - Senate amendment. Amends a Senate bill.
    Samdt,
    /// Only available for the 97th and 98th Congresses.
    Suamdt,
}

impl CongressionalAmendmentType {
    fn as_str(self) -> &'static str {
        match self {
            CongressionalAmendmentType::Hamdt => "hamdt",
            CongressionalAmendmentType::Samdt => "samdt",
            CongressionalAmendmentType::Suamdt => "suamdt",
        }
    }
}
