#![allow(clippy::module_inception)]

//! Committee API endpoints and types.

use serde::{Deserialize, Serialize};

mod bills;
mod chamber;
mod chamber_by_congress;
mod committee;
mod committee_code;
mod congress;
mod house_communication;
mod nominations;
mod reports;
mod senate_communication;

pub use self::bills::{Bills, BillsBuilder, BillsBuilderError};
pub use self::chamber::{Chamber, ChamberBuilder, ChamberBuilderError};
pub use self::chamber_by_congress::{
    ChamberByCongress, ChamberByCongressBuilder, ChamberByCongressBuilderError,
};
pub use self::committee::{Committee, CommitteeBuilder, CommitteeBuilderError};
pub use self::committee_code::{CommitteeCode, CommitteeCodeBuilder, CommitteeCodeBuilderError};
pub use self::congress::{Congress, CongressBuilder, CongressBuilderError};
pub use self::house_communication::{
    HouseCommunication, HouseCommunicationBuilder, HouseCommunicationBuilderError,
};
pub use self::nominations::{Nominations, NominationsBuilder, NominationsBuilderError};
pub use self::reports::{Reports, ReportsBuilder, ReportsBuilderError};
pub use self::senate_communication::{
    SenateCommunication, SenateCommunicationBuilder, SenateCommunicationBuilderError,
};

/// Chamber options for the Committee resource.
///
/// This differs from CommitteeChamber enum with its Joint variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommitteeChamber {
    House,
    Senate,
    Joint,
}

impl CommitteeChamber {
    fn as_str(self) -> &'static str {
        match self {
            CommitteeChamber::House => "house",
            CommitteeChamber::Senate => "senate",
            CommitteeChamber::Joint => "joint",
        }
    }
}
