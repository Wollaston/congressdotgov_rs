use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::endpoint::Endpoint, api::params::QueryParams};

use super::HouseCommunicationType;

/// Represents the /house-communication/:congress/:communicationType/:communicationNumber endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct CommunicationNumber {
    #[builder(setter(into))]
    congress: u16,
    #[builder(setter(into))]
    communication_type: HouseCommunicationType,
    #[builder(setter(into))]
    communication_number: u32,
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
            "house-communication/{}/{}/{}",
            self.congress,
            self.communication_type.as_str(),
            self.communication_number
        )
        .into()
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
        CommunicationNumber::builder()
            .congress(117_u16)
            .communication_type(HouseCommunicationType::Ec)
            .communication_number(3324_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let req_client = reqwest::Client::new();
        let client = Cdg::new(auth, req_client, Format::Json).unwrap();

        let endpoint = CommunicationNumber::builder()
            .congress(117_u16)
            .communication_type(HouseCommunicationType::Ec)
            .communication_number(3324_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
