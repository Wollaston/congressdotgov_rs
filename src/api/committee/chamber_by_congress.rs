use chrono::{DateTime, Utc};
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{
    api::{committee::CommitteeChamber, Format},
    endpoint::Endpoint,
    params::QueryParams,
};

/// Represents the /committee/:congress/:chamber endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct ChamberByCongress {
    #[builder(setter(into))]
    congress: u16,
    #[builder(setter(into))]
    chamber: CommitteeChamber,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
    #[builder(default)]
    from_date_time: Option<DateTime<Utc>>,
    #[builder(default)]
    to_date_time: Option<DateTime<Utc>>,
}

impl ChamberByCongress {
    pub fn builder() -> ChamberByCongressBuilder {
        ChamberByCongressBuilder::default()
    }
}

impl Endpoint for ChamberByCongress {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("committee/{}/{}", self.congress, self.chamber.as_str()).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("format", self.format);
        params.push_opt("offset", self.offset);
        params.push_opt("limit", self.limit);
        params.push_opt("from_date_time", self.from_date_time);
        params.push_opt("to_date_time", self.to_date_time);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::committee::CommitteeChamber, auth::Auth, cdg::Cdg, query::Query};

    use super::*;

    #[test]
    fn is_sufficient() {
        ChamberByCongress::builder()
            .congress(118_u16)
            .chamber(CommitteeChamber::House)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = ChamberByCongress::builder()
            .congress(118_u16)
            .chamber(CommitteeChamber::House)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
