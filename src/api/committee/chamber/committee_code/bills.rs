use chrono::{DateTime, Utc};
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{endpoint::Endpoint, params::QueryParams};

use super::Format;

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Bills<'a> {
    #[builder(setter(into))]
    chamber: crate::api::Chamber,
    #[builder(setter(into))]
    committee_code: Cow<'a, str>,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
    #[builder(default)]
    from_date_time: Option<DateTime<Utc>>,
    #[builder(default)]
    to_date_time: Option<DateTime<Utc>>,
}

impl<'a> Bills<'a> {
    pub fn builder() -> BillsBuilder<'a> {
        BillsBuilder::default()
    }
}

impl Endpoint for Bills<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "committee/{}/{}/bills",
            self.chamber.as_str(),
            self.committee_code
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("format", self.format);
        params.push_opt("offset", self.offset);
        params.push_opt("limit", self.limit);
        params.push_opt("from_date_time", self.from_date_time);
        params.push_opt("to_date_time", self.to_date_time);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::committee::chamber::committee_code::bills::Bills, auth::Auth, cdg::Cdg, query::Query,
    };

    #[test]
    fn is_sufficient() {
        Bills::builder()
            .chamber(crate::api::Chamber::House)
            .committee_code("hspw00")
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = Bills::builder()
            .chamber(crate::api::Chamber::House)
            .committee_code("hspw00")
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
