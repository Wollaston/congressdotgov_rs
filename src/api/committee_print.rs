use chrono::{DateTime, Utc};
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{endpoint::Endpoint, params::QueryParams};

use super::Format;

mod congress;

/// Represents the /committee-print endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct CommitteePrint {
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

impl CommitteePrint {
    pub fn builder() -> CommitteePrintBuilder {
        CommitteePrintBuilder::default()
    }
}

impl Endpoint for CommitteePrint {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "committee-print".to_string().into()
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
    use crate::{api::committee_print::CommitteePrint, auth::Auth, cdg::Cdg, query::Query};

    #[test]
    fn is_sufficient() {
        CommitteePrint::builder().build().unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = CommitteePrint::builder().build().unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
