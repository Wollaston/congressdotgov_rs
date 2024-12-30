use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{endpoint::Endpoint, params::QueryParams};

use super::Format;

/// Represents the /member/:stateCode/:district endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct District {
    #[builder(setter(into))]
    state_code: crate::api::member::StateCode,
    #[builder(setter(into))]
    district: u16,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    current_member: Option<bool>,
}

impl District {
    pub fn builder() -> DistrictBuilder {
        DistrictBuilder::default()
    }
}

impl Endpoint for District {
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
    use crate::{api::member::state_code::district::District, auth::Auth, cdg::Cdg, query::Query};

    #[test]
    fn is_sufficient() {
        District::builder()
            .state_code(crate::api::member::StateCode::MI)
            .district(10_u16)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = District::builder()
            .state_code(crate::api::member::StateCode::VA)
            .district(10_u16)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
