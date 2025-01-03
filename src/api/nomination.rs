#![allow(clippy::module_inception)]

//! Nomination API endpoints.

mod actions;
mod committees;
mod congress;
mod hearings;
mod nomination;
mod nomination_number;
mod ordinal;

pub use self::actions::{Actions, ActionsBuilder, ActionsBuilderError};
pub use self::committees::{Committees, CommitteesBuilder, CommitteesBuilderError};
pub use self::congress::{Congress, CongressBuilder, CongressBuilderError};
pub use self::hearings::{Hearings, HearingsBuilder, HearingsBuilderError};
pub use self::nomination::{Nomination, NominationBuilder, NominationBuilderError};
pub use self::nomination_number::{
    NominationNumber, NominationNumberBuilder, NominationNumberBuilderError,
};
pub use self::ordinal::{Ordinal, OrdinalBuilder, OrdinalBuilderError};
