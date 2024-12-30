use chrono::{DateTime, Utc};
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{endpoint::Endpoint, params::QueryParams};

use super::Format;

mod chamber;
mod congress;

/// Represents the /committee endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Committee {
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

impl Committee {
    pub fn builder() -> CommitteeBuilder {
        CommitteeBuilder::default()
    }
}

impl Endpoint for Committee {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "committee".to_string().into()
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

/// Chamber options for the Committee resource.
///
/// This differs from CommitteeChamber enum with its Joint variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitteeChamber {
    House,
    Senate,
    Joint,
}

impl CommitteeChamber {
    fn as_str(self) -> &'static str {
        match self {
            CommitteeChamber::House => "house",
            CommitteeChamber::Senate => "senate",
            CommitteeChamber::Joint => "joint",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::committee::Committee, auth::Auth, cdg::Cdg, query::Query};

    #[test]
    fn is_sufficient() {
        Committee::builder().build().unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = Committee::builder().build().unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
