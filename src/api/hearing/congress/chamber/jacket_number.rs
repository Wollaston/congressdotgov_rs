use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::CommitteeChamber, endpoint::Endpoint, params::QueryParams};

use super::Format;

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct JacketNumber {
    #[builder(setter(into))]
    congress: u16,
    #[builder(setter(into))]
    chamber: CommitteeChamber,
    #[builder(setter(into))]
    jacket_number: u32,
    #[builder(default)]
    format: Format,
}

impl JacketNumber {
    pub fn builder() -> JacketNumberBuilder {
        JacketNumberBuilder::default()
    }
}

impl Endpoint for JacketNumber {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "hearing/{}/{}/{}",
            self.congress,
            self.chamber.as_str(),
            self.jacket_number
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
    use crate::{
        api::{hearing::congress::chamber::jacket_number::JacketNumber, CommitteeChamber},
        auth::Auth,
        cdg::Cdg,
        query::Query,
    };

    #[test]
    fn is_sufficient() {
        JacketNumber::builder()
            .congress(116_u16)
            .chamber(CommitteeChamber::House)
            .jacket_number(41365_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = JacketNumber::builder()
            .congress(116_u16)
            .chamber(CommitteeChamber::House)
            .jacket_number(41365_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
