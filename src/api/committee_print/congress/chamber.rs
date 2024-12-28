use chrono::{DateTime, Utc};
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::CommitteeChamber, endpoint::Endpoint, params::QueryParams};

use super::Format;

mod jacket_number;

#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Chamber {
    #[builder(setter(into))]
    congress: u16,
    #[builder(setter(into))]
    chamber: CommitteeChamber,
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

impl Chamber {
    pub fn builder() -> ChamberBuilder {
        ChamberBuilder::default()
    }
}

impl Endpoint for Chamber {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "committee-print/{}/{}",
            self.congress,
            self.chamber.as_str()
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

#[cfg(test)]
mod tests {
    use crate::{
        api::committee_print::congress::chamber::Chamber, auth::Auth, cdg::Cdg, query::Query,
    };

    use crate::api::CommitteeChamber;

    #[test]
    fn is_sufficient() {
        Chamber::builder()
            .congress(117_u16)
            .chamber(CommitteeChamber::House)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = Chamber::builder()
            .congress(116_u16)
            .chamber(CommitteeChamber::House)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
