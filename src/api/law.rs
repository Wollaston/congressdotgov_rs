//! Law API endpoints and types.
//!
//! Note there is no general /law endpoint.

use serde::{Deserialize, Serialize};

pub use congress::Congress;
pub use congress::CongressBuilder;
pub use congress::CongressBuilderError;

pub use law_number::LawNumber;
pub use law_number::LawNumberBuilder;
pub use law_number::LawNumberBuilderError;

pub use law_type::LawType;
pub use law_type::LawTypeBuilder;
pub use law_type::LawTypeBuilderError;

mod congress;
mod law_number;
mod law_type;

/// The possible law types in Congress. Also known as 'slip laws.'
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CongressionalLawType {
    /// Public laws affect society as a whole
    Public,
    /// Private laws affect an individual, family, or small group
    Private,
}

impl CongressionalLawType {
    fn as_str(self) -> &'static str {
        match self {
            CongressionalLawType::Public => "pub",
            CongressionalLawType::Private => "priv",
        }
    }
}

impl Default for CongressionalLawType {
    fn default() -> Self {
        Self::Public
    }
}
