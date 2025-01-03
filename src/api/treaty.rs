#![allow(clippy::module_inception)]

//! Treaty API endpoints.

mod committees;
mod congress;
mod treaty;
mod treaty_number;
mod treaty_number_actions;
mod treaty_suffix;
mod treaty_suffix_actions;

pub use self::committees::Committees;
pub use self::committees::CommitteesBuilder;
pub use self::committees::CommitteesBuilderError;

pub use self::congress::Congress;
pub use self::congress::CongressBuilder;
pub use self::congress::CongressBuilderError;

pub use self::treaty::Treaty;
pub use self::treaty::TreatyBuilder;
pub use self::treaty::TreatyBuilderError;

pub use self::treaty_number::TreatyNumber;
pub use self::treaty_number::TreatyNumberBuilder;
pub use self::treaty_number::TreatyNumberBuilderError;

pub use self::treaty_number_actions::TreatyNumberActions;
pub use self::treaty_number_actions::TreatyNumberActionsBuilder;
pub use self::treaty_number_actions::TreatyNumberActionsBuilderError;

pub use self::treaty_suffix::TreatySuffix;
pub use self::treaty_suffix::TreatySuffixBuilder;
pub use self::treaty_suffix::TreatySuffixBuilderError;

pub use self::treaty_suffix_actions::TreatySuffixActions;
pub use self::treaty_suffix_actions::TreatySuffixActionsBuilder;
pub use self::treaty_suffix_actions::TreatySuffixActionsBuilderError;
