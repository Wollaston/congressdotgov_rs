use chrono::{DateTime, Utc};
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{endpoint::Endpoint, params::QueryParams};

use super::Format;

mod amendment_number;

/// Represents the /amendment/:congress/:amendmentType endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct AmendmentType {
    #[builder(setter(into))]
    congress: u8,
    #[builder(setter(into))]
    amendment_type: CongressionalAmendmentType,
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

impl AmendmentType {
    pub fn builder() -> AmendmentTypeBuilder {
        AmendmentTypeBuilder::default()
    }
}

impl Endpoint for AmendmentType {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "amendment/{}/{}",
            self.congress,
            self.amendment_type.as_str()
        )
        .into()
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

/// The possible Amendment Types in Congress.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CongressionalAmendmentType {
    Hamdt,
    Samdt,
    Suamdt,
}

impl CongressionalAmendmentType {
    fn as_str(self) -> &'static str {
        match self {
            CongressionalAmendmentType::Hamdt => "hamdt",
            CongressionalAmendmentType::Samdt => "samdt",
            CongressionalAmendmentType::Suamdt => "suamdt",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::amendments::congress::amendment_type::AmendmentType, auth::Auth, cdg::Cdg,
        query::Query,
    };

    use super::CongressionalAmendmentType;

    #[test]
    fn is_sufficient() {
        AmendmentType::builder()
            .congress(117_u8)
            .amendment_type(CongressionalAmendmentType::Suamdt)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = AmendmentType::builder()
            .congress(117_u8)
            .amendment_type(CongressionalAmendmentType::Suamdt)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
