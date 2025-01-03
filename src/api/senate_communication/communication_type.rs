use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::Format, api::endpoint::Endpoint, api::params::QueryParams};

use super::SenateCommunicationType;

/// Represents the /senate-communication/:congress/:communicationType endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct CommunicationType {
    #[builder(default)]
    congress: u8,
    #[builder(setter(into))]
    communication_type: SenateCommunicationType,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl CommunicationType {
    pub fn builder() -> CommunicationTypeBuilder {
        CommunicationTypeBuilder::default()
    }
}

impl Endpoint for CommunicationType {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "senate-communication/{}/{}",
            self.congress,
            self.communication_type.as_str()
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
    use crate::{auth::Auth, cdg::Cdg, api::query::Query};

    use super::*;

    #[test]
    fn is_sufficient() {
        CommunicationType::builder()
            .congress(117_u8)
            .communication_type(SenateCommunicationType::Ec)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = CommunicationType::builder()
            .congress(117_u8)
            .communication_type(SenateCommunicationType::Ec)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
