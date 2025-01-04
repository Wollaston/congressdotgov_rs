use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::common::Format, api::endpoint::Endpoint, api::params::QueryParams};

/// Represents the /member/:stateCode endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct StateCode {
    #[builder(setter(into))]
    state_code: crate::api::member::CongressionalStateCode,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    current_member: Option<bool>,
}

impl StateCode {
    pub fn builder() -> StateCodeBuilder {
        StateCodeBuilder::default()
    }
}

impl Endpoint for StateCode {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("member/{}", self.state_code.as_str()).into()
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
    use crate::{auth::Auth, cdg::Cdg, api::query::Query};

    use super::*;

    #[test]
    fn is_sufficient() {
        StateCode::builder()
            .state_code(crate::api::member::CongressionalStateCode::VA)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = StateCode::builder()
            .state_code(crate::api::member::CongressionalStateCode::VA)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
