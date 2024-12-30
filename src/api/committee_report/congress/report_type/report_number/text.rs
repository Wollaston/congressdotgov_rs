use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{
    api::committee_report::congress::report_type::CommitteeReportType, endpoint::Endpoint,
    params::QueryParams,
};

use super::Format;

/// Represents the /committee-report/:congress/:reportType/:reportNumber/text endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Text {
    #[builder(setter(into))]
    congress: u16,
    #[builder(setter(into))]
    report_type: CommitteeReportType,
    #[builder(setter(into))]
    report_number: u32,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl Text {
    pub fn builder() -> TextBuilder {
        TextBuilder::default()
    }
}

impl Endpoint for Text {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "committee-report/{}/{}/{}/text",
            self.congress,
            self.report_type.as_str(),
            self.report_number
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
        api::committee_report::congress::{
            report_type::report_number::text::Text, report_type::CommitteeReportType,
        },
        auth::Auth,
        cdg::Cdg,
        query::Query,
    };

    #[test]
    fn is_sufficient() {
        Text::builder()
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

        let endpoint = Text::builder()
            .congress(118_u16)
            .report_type(CommitteeReportType::Hrpt)
            .report_number(617_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
