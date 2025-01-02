use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::Format, endpoint::Endpoint, params::QueryParams};

use super::CongressionalStateCode;

/// Represents the /member/:stateCode/:district endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct StateCodeDistrict {
    #[builder(setter(into))]
    state_code: CongressionalStateCode,
    #[builder(setter(into))]
    district: u16,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    current_member: Option<bool>,
}

impl StateCodeDistrict {
    pub fn builder() -> StateCodeDistrictBuilder {
        StateCodeDistrictBuilder::default()
    }
}

impl Endpoint for StateCodeDistrict {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("member/{}/{}", self.state_code.as_str(), self.district).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("format", self.format);
        params.push_opt("current_member", self.current_member);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{auth::Auth, cdg::Cdg, query::Query};

    use super::*;

    #[test]
    fn is_sufficient() {
        StateCodeDistrict::builder()
            .state_code(crate::api::member::CongressionalStateCode::MI)
            .district(10_u16)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = StateCodeDistrict::builder()
            .state_code(crate::api::member::CongressionalStateCode::VA)
            .district(10_u16)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
