use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::common::Format, api::endpoint::Endpoint, api::params::QueryParams};

use super::CommitteeReportType;

/// Represents the /committee-report/:congress/:reportType/:reportNumber endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct ReportNumber {
    #[builder(setter(into))]
    congress: u16,
    #[builder(setter(into))]
    report_type: CommitteeReportType,
    #[builder(setter(into))]
    report_number: u32,
    #[builder(default)]
    format: Format,
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
        format!(
            "committee-report/{}/{}/{}",
            self.congress,
            self.report_type.as_str(),
            self.report_number
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("format", self.format);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::query::Query, auth::Auth, cdg::Cdg};

    use super::*;

    #[test]
    fn is_sufficient() {
        ReportNumber::builder()
            .congress(118_u16)
            .report_type(CommitteeReportType::Hrpt)
            .report_number(617_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = ReportNumber::builder()
            .congress(118_u16)
            .report_type(CommitteeReportType::Hrpt)
            .report_number(617_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
