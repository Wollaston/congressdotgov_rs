use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::common::Format, api::endpoint::Endpoint, api::params::QueryParams};

use super::SenateCommunicationType;

/// Represents the /senate-communication/:congress/:communicationType/:communicationNumber endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct CommunicationNumber {
    #[builder(default)]
    congress: u8,
    #[builder(setter(into))]
    communication_type: SenateCommunicationType,
    #[builder(setter(into))]
    communication_number: u32,
    #[builder(default)]
    format: Format,
}

impl CommunicationNumber {
    pub fn builder() -> CommunicationNumberBuilder {
        CommunicationNumberBuilder::default()
    }
}

impl Endpoint for CommunicationNumber {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "senate-communication/{}/{}/{}",
            self.congress,
            self.communication_type.as_str(),
            self.communication_number
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
        CommunicationNumber::builder()
            .congress(117_u8)
            .communication_type(SenateCommunicationType::Ec)
            .communication_number(2561_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let req_client = reqwest::Client::new();
let client = Cdg::new(auth, req_client).unwrap();

        let endpoint = CommunicationNumber::builder()
            .congress(117_u8)
            .communication_type(SenateCommunicationType::Ec)
            .communication_number(2561_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
