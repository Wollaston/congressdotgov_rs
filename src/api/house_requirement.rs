#![allow(clippy::module_inception)]

//! House-requirement API endpoints.

mod house_requirement;
mod matching_communications;
mod requirement_number;

pub use self::house_requirement::{
    HouseRequirement, HouseRequirementBuilder, HouseRequirementBuilderError,
};
pub use self::matching_communications::{
    MatchingCommunications, MatchingCommunicationsBuilder, MatchingCommunicationsBuilderError,
};
pub use self::requirement_number::{
    RequirementNumber, RequirementNumberBuilder, RequirementNumberBuilderError,
};
