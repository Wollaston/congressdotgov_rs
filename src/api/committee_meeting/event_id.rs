use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::common::CommitteeChamber, api::endpoint::Endpoint, api::params::QueryParams};

/// Represents the /committee-meeting/:congress/:chamber/:eventId endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct EventId {
    #[builder(setter(into))]
    congress: u16,
    #[builder(setter(into))]
    chamber: CommitteeChamber,
    #[builder(setter(into))]
    event_id: u32,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl EventId {
    pub fn builder() -> EventIdBuilder {
        EventIdBuilder::default()
    }
}

impl Endpoint for EventId {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "committee-meeting/{}/{}/{}",
            self.congress,
            self.chamber.as_str(),
            self.event_id
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
        EventId::builder()
            .congress(118_u16)
            .chamber(CommitteeChamber::House)
            .event_id(115538_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let req_client = reqwest::Client::new();
        let client = Cdg::new(auth, req_client, Format::Json).unwrap();

        let endpoint = EventId::builder()
            .congress(118_u16)
            .chamber(CommitteeChamber::House)
            .event_id(115538_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
