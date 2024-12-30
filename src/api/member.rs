use chrono::{DateTime, Utc};
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{endpoint::Endpoint, params::QueryParams};

use super::Format;

mod bioguide_id;
mod congress;
mod state_code;

/// Represents the /member endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Member {
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
    #[builder(default)]
    from_date_time: Option<DateTime<Utc>>,
    #[builder(default)]
    to_date_time: Option<DateTime<Utc>>,
    #[builder(default)]
    current_member: Option<bool>,
}

impl Member {
    pub fn builder() -> MemberBuilder {
        MemberBuilder::default()
    }
}

impl Endpoint for Member {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "member".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("format", self.format);
        params.push_opt("offset", self.offset);
        params.push_opt("limit", self.limit);
        params.push_opt("from_date_time", self.from_date_time);
        params.push_opt("to_date_time", self.to_date_time);
        params.push_opt("current_member", self.current_member);

        params
    }
}

/// The different possible state codes that can be used when querying
/// and filtering Congressional member data. These match the two-digit
/// postal codes for the 50 U.S. states and the District of Columbia.
#[derive(Debug, Clone, Copy)]
pub enum StateCode {
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

impl StateCode {
    fn as_str(self) -> &'static str {
        use StateCode::*;
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

#[cfg(test)]
mod tests {
    use crate::{api::member::Member, auth::Auth, cdg::Cdg, query::Query};

    #[test]
    fn is_sufficient() {
        Member::builder().build().unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = Member::builder().build().unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
