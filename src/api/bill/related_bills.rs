use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::Format, api::endpoint::Endpoint, api::params::QueryParams};

/// Represents the /bill/:congress/:billtype/:billnumber/relatedbills endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct RelatedBills {
    #[builder(setter(into))]
    congress: u8,
    #[builder(setter(into))]
    bill_type: crate::api::BillType,
    #[builder(setter(into))]
    bill_number: u32,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl RelatedBills {
    pub fn builder() -> RelatedBillsBuilder {
        RelatedBillsBuilder::default()
    }
}

impl Endpoint for RelatedBills {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "bill/{}/{}/{}/relatedbills",
            self.congress,
            self.bill_type.as_str(),
            self.bill_number
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
    fn bll_is_sufficient() {
        RelatedBills::builder()
            .congress(117_u8)
            .bill_type(crate::api::BillType::Hr)
            .bill_number(3076_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = RelatedBills::builder()
            .congress(117_u8)
            .bill_type(crate::api::BillType::Hr)
            .bill_number(3076_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
