use chrono::{DateTime, Utc};
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{
    api::{Format, Sort},
    endpoint::Endpoint,
    params::QueryParams,
};

mod bill_type;

/// Represents the /bill/:congress endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Congress {
    #[builder(setter(into))]
    congress: u8,
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
    #[builder(default)]
    sort: Option<Sort>,
}

impl Congress {
    pub fn builder() -> CongressBuilder {
        CongressBuilder::default()
    }
}

impl Endpoint for Congress {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("bill/{}", self.congress).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("format", self.format);
        params.push_opt("offset", self.offset);
        params.push_opt("limit", self.limit);
        params.push_opt("from_date_time", self.from_date_time);
        params.push_opt("to_date_time", self.to_date_time);
        params.push_opt("sort", self.sort);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::bill::congress::Congress, auth::Auth, cdg::Cdg, query::Query};

    #[test]
    fn bll_is_sufficient() {
        Congress::builder().congress(117_u8).build().unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = Congress::builder().congress(117_u8).build().unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
