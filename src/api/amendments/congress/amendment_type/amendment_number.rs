use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use super::{CongressionalAmendmentType, Format};

use crate::{endpoint::Endpoint, params::QueryParams};

mod actions;
mod amendments;
mod cosponsors;
mod text;

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct AmendmentNumber {
    #[builder(setter(into))]
    congress: u8,
    #[builder(setter(into))]
    amendment_type: CongressionalAmendmentType,
    #[builder(setter(into))]
    amendment_number: u32,
    #[builder(default)]
    format: Format,
}

impl AmendmentNumber {
    pub fn builder() -> AmendmentNumberBuilder {
        AmendmentNumberBuilder::default()
    }
}

impl Endpoint for AmendmentNumber {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "amendment/{}/{}/{}",
            self.congress,
            self.amendment_type.as_str(),
            self.amendment_number
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("format", self.format);

        params
    }
}

#[cfg(test)]
mod tests {
    use super::CongressionalAmendmentType;

    use crate::{
        api::amendments::congress::amendment_type::amendment_number::AmendmentNumber, auth::Auth,
        cdg::Cdg, query::Query,
    };

    #[test]
    fn is_sufficient() {
        AmendmentNumber::builder()
            .congress(117_u8)
            .amendment_type(CongressionalAmendmentType::Samdt)
            .amendment_number(2137_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = AmendmentNumber::builder()
            .congress(117_u8)
            .amendment_type(CongressionalAmendmentType::Samdt)
            .amendment_number(2137_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
