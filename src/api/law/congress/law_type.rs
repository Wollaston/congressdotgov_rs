use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{
    api::{Format, LawType},
    endpoint::Endpoint, params::QueryParams,
};

mod law_number;

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct CongressByLawType {
    #[builder(setter(into))]
    congress: u16,
    #[builder(default)]
    law_type: LawType,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl CongressByLawType {
    pub fn builder() -> CongressByLawTypeBuilder {
        CongressByLawTypeBuilder::default()
    }
}

impl Endpoint for CongressByLawType {
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
    use crate::{
        api::law::congress::law_type::CongressByLawType, auth::Auth, cdg::Cdg, query::Query,
    };

    use super::LawType;

    #[test]
    fn is_sufficient() {
        CongressByLawType::builder()
            .congress(118_u16)
            .law_type(super::LawType::Public)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = CongressByLawType::builder()
            .congress(118_u16)
            .law_type(LawType::Public)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
