use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{endpoint::Endpoint, params::QueryParams};

use super::Format;

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct CosponsoredLegislation<'a> {
    #[builder(setter(into))]
    bioguide_id: Cow<'a, str>,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl<'a> CosponsoredLegislation<'a> {
    pub fn builder() -> CosponsoredLegislationBuilder<'a> {
        CosponsoredLegislationBuilder::default()
    }
}

impl Endpoint for CosponsoredLegislation<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("member/{}/sponsored-legislation", self.bioguide_id).into()
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
        api::member::bioguide_id::cosponsored_legislation::CosponsoredLegislation, auth::Auth,
        cdg::Cdg, query::Query,
    };

    #[test]
    fn is_sufficient() {
        CosponsoredLegislation::builder()
            .bioguide_id("L000174")
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = CosponsoredLegislation::builder()
            .bioguide_id("L000174")
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
