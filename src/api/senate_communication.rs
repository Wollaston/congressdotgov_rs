#![allow(clippy::module_inception)]

//! Senate-communication API endpoints and types.

use serde::{Deserialize, Serialize};

mod communication_number;
mod communication_type;
mod congress;
mod senate_communication;

pub use self::communication_number::{
    CommunicationNumber, CommunicationNumberBuilder, CommunicationNumberBuilderError,
};
pub use self::communication_type::{
    CommunicationType, CommunicationTypeBuilder, CommunicationTypeBuilderError,
};
pub use self::congress::{Congress, CongressBuilder, CongressBuilderError};
pub use self::senate_communication::{
    SenateCommunication, SenateCommunicationBuilder, SenateCommunicationBuilderError,
};

/// The different Senate Communication Types.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SenateCommunicationType {
    /// Executive Communications
    Ec,
    /// Presidential Messages
    Pm,
    /// Petitions or Memorials
    Pom,
}

impl SenateCommunicationType {
    fn as_str(self) -> &'static str {
        match self {
            SenateCommunicationType::Ec => "ec",
            SenateCommunicationType::Pm => "pm",
            SenateCommunicationType::Pom => "pom",
        }
    }
}
