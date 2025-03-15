use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::{
    common::BillType,
    endpoint::Endpoint,
    params::QueryParams,
};

/// Represents the /bill/:congress/:billtype/:billnumber/actions endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Actions {
    #[builder(setter(into))]
    congress: u8,
    #[builder(setter(into))]
    bill_type: BillType,
    #[builder(setter(into))]
    bill_number: u32,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl Actions {
    pub fn builder() -> ActionsBuilder {
        ActionsBuilder::default()
    }
}

impl Endpoint for Actions {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "bill/{}/{}/{}/actions",
            self.congress,
            self.bill_type.as_str(),
            self.bill_number
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push_opt("offset", self.offset);
        params.push_opt("limit", self.limit);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::common::Format, api::query::Query, auth::Auth, cdg::Cdg};

    use super::*;

    #[test]
    fn bll_is_sufficient() {
        Actions::builder()
            .congress(117_u8)
            .bill_type(crate::api::common::BillType::Hr)
            .bill_number(3076_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let req_client = reqwest::Client::new();
        let client = Cdg::new(auth, req_client, Format::Json).unwrap();

        let endpoint = Actions::builder()
            .congress(117_u8)
            .bill_type(crate::api::common::BillType::Hr)
            .bill_number(3076_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
