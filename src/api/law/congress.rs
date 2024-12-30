use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::Format, endpoint::Endpoint, params::QueryParams};

mod law_type;

/// Represents the /law/:congress endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Congress {
    #[builder(setter(into))]
    congress: u16,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl Congress {
    pub fn builder() -> CongressBuilder {
        CongressBuilder::default()
    }
}

impl Endpoint for Congress {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("law/{}", self.congress).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("format", self.format);
        params.push_opt("offset", self.offset);
        params.push_opt("limit", self.limit);

        params
    }
}

/// The possible law types in Congress. Also known as 'slip laws.'
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LawType {
    /// Public laws affect society as a whole
    Public,
    /// Private laws affect an individual, family, or small group
    Private,
}

impl LawType {
    fn as_str(self) -> &'static str {
        match self {
            LawType::Public => "pub",
            LawType::Private => "priv",
        }
    }
}

impl Default for LawType {
    fn default() -> Self {
        Self::Public
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::law::congress::Congress, auth::Auth, cdg::Cdg, query::Query};

    #[test]
    fn is_sufficient() {
        Congress::builder().congress(118_u16).build().unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = Congress::builder().congress(118_u16).build().unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
