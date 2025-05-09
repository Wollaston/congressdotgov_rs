use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::committee::CommitteeChamber, api::endpoint::Endpoint, api::params::QueryParams};

/// Represents the /committee/:chamber/:committeeCode endpoint.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct CommitteeCode<'a> {
    #[builder(setter(into))]
    chamber: CommitteeChamber,
    #[builder(setter(into))]
    committee_code: Cow<'a, str>,
}

impl<'a> CommitteeCode<'a> {
    pub fn builder() -> CommitteeCodeBuilder<'a> {
        CommitteeCodeBuilder::default()
    }
}

impl Endpoint for CommitteeCode<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "committee/{}/{}",
            self.chamber.as_str(),
            self.committee_code
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::{committee::CommitteeChamber, common::Format, query::Query},
        auth::Auth,
        cdg::Cdg,
    };

    use super::*;

    #[test]
    fn is_sufficient() {
        CommitteeCode::builder()
            .chamber(CommitteeChamber::House)
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

        let endpoint = CommitteeCode::builder()
            .chamber(CommitteeChamber::House)
            .committee_code("hspw00")
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
