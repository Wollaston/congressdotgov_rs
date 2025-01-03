use chrono::{DateTime, Utc};
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::Format, api::endpoint::Endpoint, api::params::QueryParams};

/// Represents the /committee-report endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct CommitteeReport {
    #[builder(default)]
    format: Format,
    #[builder(default)]
    conference: Option<bool>,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
    #[builder(default)]
    from_date_time: Option<DateTime<Utc>>,
    #[builder(default)]
    to_date_time: Option<DateTime<Utc>>,
}

impl CommitteeReport {
    pub fn builder() -> CommitteeReportBuilder {
        CommitteeReportBuilder::default()
    }
}

impl Endpoint for CommitteeReport {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "committee-report".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("format", self.format);
        params.push_opt("conference", self.conference);
        params.push_opt("offset", self.offset);
        params.push_opt("limit", self.limit);
        params.push_opt("from_date_time", self.from_date_time);
        params.push_opt("to_date_time", self.to_date_time);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{auth::Auth, cdg::Cdg, api::query::Query};

    use super::*;

    #[test]
    fn is_sufficient() {
        CommitteeReport::builder().build().unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = CommitteeReport::builder().build().unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
