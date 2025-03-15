use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{
    api::endpoint::Endpoint,
    api::params::QueryParams,
    api::committee::CommitteeChamber,
};

/// Represents the /committee/:chamber/:committeeCode/senate-communication endpoint.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct SenateCommunication<'a> {
    #[builder(setter(into))]
    chamber: CommitteeChamber,
    #[builder(setter(into))]
    committee_code: Cow<'a, str>,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl<'a> SenateCommunication<'a> {
    pub fn builder() -> SenateCommunicationBuilder<'a> {
        SenateCommunicationBuilder::default()
    }
}

impl Endpoint for SenateCommunication<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "committee/{}/{}/senate-communication",
            self.chamber.as_str(),
            self.committee_code
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push_opt("offset", self.offset);
        params.push_opt("limit", self.limit);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::common::Format, api::query::Query, auth::Auth, cdg::Cdg};

    use super::*;

    #[test]
    fn is_sufficient() {
        SenateCommunication::builder()
            .chamber(CommitteeChamber::Senate)
            .committee_code("hspw00")
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let req_client = reqwest::Client::new();
        let client = Cdg::new(auth, req_client, Format::Json).unwrap();

        let endpoint = SenateCommunication::builder()
            .chamber(CommitteeChamber::Senate)
            .committee_code("ssas00")
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
