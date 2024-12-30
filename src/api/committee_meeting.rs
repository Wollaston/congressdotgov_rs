use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{endpoint::Endpoint, params::QueryParams};

use super::Format;

mod congress;

/// Represents the /committee-meeting endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct CommitteeMeeting {
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl CommitteeMeeting {
    pub fn builder() -> CommitteeMeetingBuilder {
        CommitteeMeetingBuilder::default()
    }
}

impl Endpoint for CommitteeMeeting {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "committee-meeting".to_string().into()
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
    use crate::{api::committee_meeting::CommitteeMeeting, auth::Auth, cdg::Cdg, query::Query};

    #[test]
    fn is_sufficient() {
        CommitteeMeeting::builder().build().unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = CommitteeMeeting::builder().build().unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
