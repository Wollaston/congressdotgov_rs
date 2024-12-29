use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{endpoint::Endpoint, params::QueryParams};

use super::Format;

mod actions;
mod committees;
mod treaty_suffix;

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct TreatyNumber {
    #[builder(setter(into))]
    congress: u8,
    #[builder(setter(into))]
    treaty_number: u32,
    #[builder(default)]
    format: Format,
}

impl TreatyNumber {
    pub fn builder() -> TreatyNumberBuilder {
        TreatyNumberBuilder::default()
    }
}

impl Endpoint for TreatyNumber {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("treaty/{}/{}", self.congress, self.treaty_number).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("format", self.format);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::treaty::congress::treaty_number::TreatyNumber, auth::Auth, cdg::Cdg, query::Query,
    };

    #[test]
    fn is_sufficient() {
        TreatyNumber::builder()
            .congress(117_u8)
            .treaty_number(3_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = TreatyNumber::builder()
            .congress(117_u8)
            .treaty_number(3_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
