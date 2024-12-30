use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{endpoint::Endpoint, params::QueryParams};

use super::Format;

mod cosponsored_legislation;
mod sponsored_legislation;

/// Represents the /member/:bioguideId endpoint.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct BioguideId<'a> {
    #[builder(setter(into))]
    bioguide_id: Cow<'a, str>,
    #[builder(default)]
    format: Format,
}

impl<'a> BioguideId<'a> {
    pub fn builder() -> BioguideIdBuilder<'a> {
        BioguideIdBuilder::default()
    }
}

impl Endpoint for BioguideId<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("member/{}", self.bioguide_id).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("format", self.format);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::member::bioguide_id::BioguideId, auth::Auth, cdg::Cdg, query::Query};

    #[test]
    fn is_sufficient() {
        BioguideId::builder()
            .bioguide_id("L000174")
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = BioguideId::builder()
            .bioguide_id("L000174")
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
