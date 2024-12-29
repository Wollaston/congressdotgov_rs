use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{endpoint::Endpoint, params::QueryParams};

use super::Format;

mod communication_number;

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

#[derive(Debug, Clone, Copy)]
enum SenateCommunicationType {
    Ec,
    Pm,
    Pom,
}

impl SenateCommunicationType {
    fn as_str(self) -> &'static str {
        match self {
            SenateCommunicationType::Ec => "ec",
            SenateCommunicationType::Pm => "pm",
            SenateCommunicationType::Pom => "pom",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::senate_communication::congress::communication_type::CommunicationType, auth::Auth,
        cdg::Cdg, query::Query,
    };

    use super::SenateCommunicationType;

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
