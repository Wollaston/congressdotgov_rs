use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{endpoint::Endpoint, params::QueryParams};

use super::Format;

mod day;

/// Represents the /bound-congressional-record/:year/:month endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Month {
    #[builder(setter(into))]
    year: u16,
    #[builder(setter(into))]
    month: u8,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl Month {
    pub fn builder() -> MonthBuilder {
        MonthBuilder::default()
    }
}

impl Endpoint for Month {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("bound-congressional-record/{}/{}", self.year, self.month).into()
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
    use crate::{
        api::bound_congressional_record::year::month::Month, auth::Auth, cdg::Cdg, query::Query,
    };

    #[test]
    fn is_sufficient() {
        Month::builder().year(1990_u16).month(4_u8).build().unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = Month::builder().year(1990_u16).month(5_u8).build().unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
