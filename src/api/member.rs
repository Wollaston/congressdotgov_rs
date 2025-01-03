#![allow(clippy::module_inception)]

//! Member API endpoints and types.

mod bioguide_id;
mod congress;
mod congress_state_code_district;
mod cosponsored_legislation;
mod member;
mod sponsored_legislation;
mod state_code;
mod state_code_district;

pub use self::bioguide_id::{BioguideId, BioguideIdBuilder, BioguideIdBuilderError};
pub use self::congress::{Congress, CongressBuilder, CongressBuilderError};
pub use self::congress_state_code_district::{
    CongressStateCodeDistrict, CongressStateCodeDistrictBuilder,
    CongressStateCodeDistrictBuilderError,
};
pub use self::cosponsored_legislation::{
    CosponsoredLegislation, CosponsoredLegislationBuilder, CosponsoredLegislationBuilderError,
};
pub use self::member::{Member, MemberBuilder, MemberBuilderError};
pub use self::sponsored_legislation::{
    SponsoredLegislation, SponsoredLegislationBuilder, SponsoredLegislationBuilderError,
};
pub use self::state_code::{StateCode, StateCodeBuilder, StateCodeBuilderError};
pub use self::state_code_district::{
    StateCodeDistrict, StateCodeDistrictBuilder, StateCodeDistrictBuilderError,
};

/// The different possible state codes that can be used when querying
/// and filtering Congressional member data. These match the two-digit
/// postal codes for the 50 U.S. states and the District of Columbia.
#[derive(Debug, Clone, Copy)]
pub enum CongressionalStateCode {
    AL,
    AK,
    AZ,
    AR,
    CA,
    CO,
    CT,
    DE,
    DC,
    FL,
    GA,
    HI,
    ID,
    IL,
    IN,
    IA,
    KS,
    KY,
    LA,
    ME,
    MD,
    MA,
    MI,
    MN,
    MS,
    MO,
    MT,
    NE,
    NV,
    NH,
    NJ,
    NM,
    NY,
    NC,
    ND,
    OH,
    OK,
    OR,
    PA,
    RI,
    SC,
    SD,
    TN,
    TX,
    UT,
    VT,
    VA,
    WA,
    WV,
    WI,
    WY,
}

impl CongressionalStateCode {
    fn as_str(self) -> &'static str {
        use CongressionalStateCode::*;
        match self {
            AL => "AL",
            AK => "AK",
            AZ => "AZ",
            AR => "AR",
            CA => "CA",
            CO => "CO",
            CT => "CT",
            DE => "DE",
            DC => "DC",
            FL => "FL",
            GA => "GA",
            HI => "HI",
            ID => "ID",
            IL => "IL",
            IN => "IN",
            IA => "IA",
            KS => "KS",
            KY => "KY",
            LA => "LA",
            ME => "ME",
            MD => "MD",
            MA => "MA",
            MI => "MI",
            MN => "MN",
            MS => "MS",
            MO => "MO",
            MT => "MT",
            NE => "NE",
            NV => "NV",
            NH => "NH",
            NJ => "NJ",
            NM => "NM",
            NY => "NY",
            NC => "NC",
            ND => "ND",
            OH => "OH",
            OK => "OK",
            OR => "OR",
            PA => "PA",
            RI => "RI",
            SC => "SC",
            SD => "SD",
            TN => "TN",
            TX => "TX",
            UT => "UT",
            VT => "VT",
            VA => "VA",
            WA => "WA",
            WV => "WV",
            WI => "WI",
            WY => "WY",
        }
    }
}
