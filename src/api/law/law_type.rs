use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{
    api::endpoint::Endpoint,
    api::params::QueryParams,
    api::{common::Format, law::CongressionalLawType},
};

/// Represents the /law/:congress/:lawType endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct LawType {
    #[builder(setter(into))]
    congress: u16,
    #[builder(default)]
    law_type: CongressionalLawType,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl LawType {
    pub fn builder() -> LawTypeBuilder {
        LawTypeBuilder::default()
    }
}

impl Endpoint for LawType {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("law/{}/{}", self.congress, self.law_type.as_str()).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("format", self.format);
        params.push_opt("offset", self.offset);
        params.push_opt("limit", self.limit);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::query::Query, auth::Auth, Cdg};

    use super::*;

    #[test]
    fn is_sufficient() {
        LawType::builder()
            .congress(118_u16)
            .law_type(CongressionalLawType::Public)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = LawType::builder()
            .congress(118_u16)
            .law_type(CongressionalLawType::Public)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
