use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::endpoint::Endpoint, api::params::QueryParams};

/// Represents the /crsreport/:reportNumber endpoint.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ReportNumber {
    #[builder(setter(into))]
    report_number: Cow<'static, str>,
}

impl ReportNumber {
    pub fn builder() -> ReportNumberBuilder {
        ReportNumberBuilder::default()
    }
}

impl Endpoint for ReportNumber {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("crsreport/{}", self.report_number).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::common::Format, api::query::Query, auth::Auth, cdg::Cdg};

    use super::*;

    #[test]
    fn is_sufficient() {
        ReportNumber::builder()
            .report_number("R47175")
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let req_client = reqwest::Client::new();
        let client = Cdg::new(auth, req_client, Format::Json).unwrap();

        let endpoint = ReportNumber::builder()
            .report_number("R47175")
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
