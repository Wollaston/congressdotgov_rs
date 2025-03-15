use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{
    api::endpoint::Endpoint,
    api::params::QueryParams,
    api::member::CongressionalStateCode,
};

/// Represents the /member/congress/:congress/:stateCode/:district endpoint.
///
/// There is no /member/congress/:congress/:stateCode endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct CongressStateCodeDistrict {
    #[builder(setter(into))]
    congress: u16,
    #[builder(setter(into))]
    state_code: CongressionalStateCode,
    #[builder(setter(into))]
    district: u16,
    #[builder(default)]
    current_member: Option<bool>,
}

impl CongressStateCodeDistrict {
    pub fn builder() -> CongressStateCodeDistrictBuilder {
        CongressStateCodeDistrictBuilder::default()
    }
}

impl Endpoint for CongressStateCodeDistrict {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "member/congress/{}/{}/{}",
            self.congress,
            self.state_code.as_str(),
            self.district
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push_opt("current_member", self.current_member);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::common::Format, api::query::Query, auth::Auth, cdg::Cdg};

    use super::*;

    #[test]
    fn is_sufficient() {
        CongressStateCodeDistrict::builder()
            .congress(118_u16)
            .state_code(CongressionalStateCode::MI)
            .district(10_u16)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let req_client = reqwest::Client::new();
        let client = Cdg::new(auth, req_client, Format::Json).unwrap();

        let endpoint = CongressStateCodeDistrict::builder()
            .congress(118_u16)
            .state_code(CongressionalStateCode::MI)
            .district(10_u16)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
