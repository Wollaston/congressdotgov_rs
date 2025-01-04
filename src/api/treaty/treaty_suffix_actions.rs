use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::common::Format, api::endpoint::Endpoint, api::params::QueryParams};

/// Represents the /treaty/:congress/:treatyNumber/:treatySuffix/actions endpoint.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct TreatySuffixActions<'a> {
    #[builder(setter(into))]
    congress: u8,
    #[builder(setter(into))]
    treaty_number: u32,
    #[builder(setter(into))]
    treaty_suffix: Cow<'a, str>,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl<'a> TreatySuffixActions<'a> {
    pub fn builder() -> TreatySuffixActionsBuilder<'a> {
        TreatySuffixActionsBuilder::default()
    }
}

impl Endpoint for TreatySuffixActions<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "treaty/{}/{}/{}/actions",
            self.congress, self.treaty_number, self.treaty_suffix
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
    use crate::{api::query::Query, auth::Auth, cdg::Cdg};

    use super::*;

    #[test]
    fn is_sufficient() {
        TreatySuffixActions::builder()
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
        let client = Cdg::new(auth).unwrap();

        let endpoint = TreatySuffixActions::builder()
            .congress(114_u8)
            .treaty_number(13_u32)
            .treaty_suffix("A")
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
