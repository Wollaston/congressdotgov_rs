#![allow(clippy::module_inception)]

//! House-communication API endpoints and types.

mod communication_number;
mod communication_type;
mod congress;
mod house_communication;

/// The possible communication types in the House of Representatives
/// available via the congress.gov API. R – Requirements also exists,
/// but it is not an option with the API.
#[derive(Debug, Clone, Copy)]
pub enum HouseCommunicationType {
    /// Executive Communications
    Ec,
    /// Memorials
    Ml,
    /// Presidential Messages
    Pm,
    /// Petitions
    Pt,
}

impl HouseCommunicationType {
    fn as_str(self) -> &'static str {
        match self {
            HouseCommunicationType::Ec => "ec",
            HouseCommunicationType::Ml => "ml",
            HouseCommunicationType::Pm => "pm",
            HouseCommunicationType::Pt => "pt",
        }
    }
}

pub use self::communication_number::{
    CommunicationNumber, CommunicationNumberBuilder, CommunicationNumberBuilderError,
};
pub use self::communication_type::{
    CommunicationType, CommunicationTypeBuilder, CommunicationTypeBuilderError,
};
pub use self::congress::{Congress, CongressBuilder, CongressBuilderError};
pub use self::house_communication::{
    HouseCommunication, HouseCommunicationBuilder, HouseCommunicationBuilderError,
};
