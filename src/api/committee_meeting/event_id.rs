use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{
    api::{CommitteeChamber, Format},
    api::endpoint::Endpoint,
    api::params::QueryParams,
};

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
    format: Format,
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
        let client = Cdg::new(auth).unwrap();

        let endpoint = EventId::builder()
            .congress(118_u16)
            .chamber(CommitteeChamber::House)
            .event_id(115538_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
