use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::Format, endpoint::Endpoint, params::QueryParams};

mod actions;
mod amendments;
mod committees;
mod cosponsors;
mod related_bills;
mod subjects;
mod summaries;
mod text;
mod titles;

/// Represents the /bill/:congress/:billtype/:billnumber endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct BillNumber {
    #[builder(setter(into))]
    congress: u8,
    #[builder(setter(into))]
    bill_type: crate::api::BillType,
    #[builder(setter(into))]
    bill_number: u32,
    #[builder(default)]
    format: Format,
}

impl BillNumber {
    pub fn builder() -> BillNumberBuilder {
        BillNumberBuilder::default()
    }
}

impl Endpoint for BillNumber {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "bill/{}/{}/{}",
            self.congress,
            self.bill_type.as_str(),
            self.bill_number
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
        api::bill::congress::bill_type::bill_number::BillNumber, auth::Auth, cdg::Cdg, query::Query,
    };

    #[test]
    fn bll_is_sufficient() {
        BillNumber::builder()
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

        let endpoint = BillNumber::builder()
            .congress(117_u8)
            .bill_type(crate::api::BillType::Hr)
            .bill_number(3076_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
