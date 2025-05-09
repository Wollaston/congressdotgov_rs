use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::endpoint::Endpoint, api::params::QueryParams};

/// Represents the /treaty/:congress/:treatyNumber/:treatySuffix endpoint.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct TreatySuffix<'a> {
    #[builder(setter(into))]
    congress: u8,
    #[builder(setter(into))]
    treaty_number: u32,
    #[builder(setter(into))]
    treaty_suffix: Cow<'a, str>,
}

impl<'a> TreatySuffix<'a> {
    pub fn builder() -> TreatySuffixBuilder<'a> {
        TreatySuffixBuilder::default()
    }
}

impl Endpoint for TreatySuffix<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "treaty/{}/{}/{}",
            self.congress, self.treaty_number, self.treaty_suffix
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::{common::Format, query::Query},
        auth::Auth,
        cdg::Cdg,
    };

    use super::*;

    #[test]
    fn is_sufficient() {
        TreatySuffix::builder()
            .congress(114_u8)
            .treaty_number(13_u32)
            .treaty_suffix("A")
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let req_client = reqwest::Client::new();
        let client = Cdg::new(auth, req_client, Format::Json).unwrap();

        let endpoint = TreatySuffix::builder()
            .congress(114_u8)
            .treaty_number(13_u32)
            .treaty_suffix("A")
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
