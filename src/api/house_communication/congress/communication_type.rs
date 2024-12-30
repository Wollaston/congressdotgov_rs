use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{endpoint::Endpoint, params::QueryParams};

use super::Format;

mod communication_number;

/// Represents the /house-communication/:congress/:communicationType endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct CommunicationType {
    #[builder(setter(into))]
    congress: u16,
    #[builder(setter(into))]
    communication_type: HouseCommunicationType,
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
            "house-communication/{}/{}",
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

/// The possible communication types in the House of Representatives
/// available via the congress.gov API. R – Requirements also exists,
/// but it is not an option with the API.
#[derive(Debug, Clone, Copy)]
pub enum HouseCommunicationType {
    /// Executive Communications
    Ec,
    /// Memorials
    Ml,
    /// Presidential Messages
    Pm,
    /// Petitions
    Pt,
}

impl HouseCommunicationType {
    fn as_str(self) -> &'static str {
        match self {
            HouseCommunicationType::Ec => "ec",
            HouseCommunicationType::Ml => "ml",
            HouseCommunicationType::Pm => "pm",
            HouseCommunicationType::Pt => "pt",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::house_communication::congress::communication_type::CommunicationType, auth::Auth,
        cdg::Cdg, query::Query,
    };

    use super::HouseCommunicationType;

    #[test]
    fn is_sufficient() {
        CommunicationType::builder()
            .congress(116_u16)
            .communication_type(HouseCommunicationType::Pm)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = CommunicationType::builder()
            .congress(116_u16)
            .communication_type(HouseCommunicationType::Pm)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
