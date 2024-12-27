use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::Format, endpoint::Endpoint, params::QueryParams};

use super::LawType;

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct CongressByLawTypeByLawNumber {
    #[builder(setter(into))]
    congress: u16,
    #[builder(default)]
    law_type: LawType,
    #[builder(default)]
    law_number: u32,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl CongressByLawTypeByLawNumber {
    pub fn builder() -> CongressByLawTypeByLawNumberBuilder {
        CongressByLawTypeByLawNumberBuilder::default()
    }
}

impl Endpoint for CongressByLawTypeByLawNumber {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "law/{}/{}/{}",
            self.congress,
            self.law_type.as_str(),
            self.law_number
        )
        .into()
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
        api::law::congress::law_type::law_number::CongressByLawTypeByLawNumber, auth::Auth,
        cdg::Cdg, query::Query,
    };

    use super::LawType;

    #[test]
    fn is_sufficient() {
        CongressByLawTypeByLawNumber::builder()
            .congress(118_u16)
            .law_type(super::LawType::Public)
            .law_number(108)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = CongressByLawTypeByLawNumber::builder()
            .congress(118_u16)
            .law_type(LawType::Public)
            .law_number(108)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
