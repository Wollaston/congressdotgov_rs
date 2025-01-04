use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::endpoint::Endpoint, api::params::QueryParams, api::Format};

/// Represents the /nomination/:congress/:nominationNumber/:ordinal endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Ordinal {
    #[builder(setter(into))]
    congress: u8,
    #[builder(setter(into))]
    nomination_number: u32,
    #[builder(setter(into))]
    ordinal: u32,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl Ordinal {
    pub fn builder() -> OrdinalBuilder {
        OrdinalBuilder::default()
    }
}

impl Endpoint for Ordinal {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "nomination/{}/{}/{}",
            self.congress, self.nomination_number, self.ordinal
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
    use crate::{api::query::Query, auth::Auth, cdg::Cdg};

    use super::*;

    #[test]
    fn defaults_are_sufficient() {
        Ordinal::builder()
            .congress(117_u8)
            .nomination_number(2467_u32)
            .ordinal(1_u32)
            .build()
            .unwrap();
    }

    #[test]
    fn congress_is_necessary() {
        let err = Ordinal::builder()
            .nomination_number(2467_u32)
            .ordinal(1_u32)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, OrdinalBuilderError, "congress");
    }

    #[test]
    fn nomination_number_is_necessary() {
        let err = Ordinal::builder()
            .congress(117_u8)
            .ordinal(1_u32)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, OrdinalBuilderError, "nomination_number");
    }

    #[test]
    fn ordinal_is_necessary() {
        let err = Ordinal::builder()
            .congress(117_u8)
            .nomination_number(2467_u32)
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, OrdinalBuilderError, "ordinal");
    }

    #[test]
    fn test_ordinal_format_builder() {
        Ordinal::builder()
            .format(Format::Xml)
            .congress(117_u8)
            .nomination_number(2467_u32)
            .ordinal(1_u32)
            .build()
            .unwrap();
    }

    #[test]
    fn test_ordinal_offset_builder() {
        Ordinal::builder()
            .congress(117_u8)
            .nomination_number(2467_u32)
            .ordinal(1_u32)
            .offset(10_u32)
            .build()
            .unwrap();
    }

    #[test]
    fn test_ordinal_limit_builder() {
        Ordinal::builder()
            .congress(117_u8)
            .nomination_number(2467_u32)
            .ordinal(1_u32)
            .limit(10_u8)
            .build()
            .unwrap();
    }

    #[test]
    fn test_ordinal_limit_offest_builder() {
        Ordinal::builder()
            .congress(117_u8)
            .nomination_number(2467_u32)
            .ordinal(1_u32)
            .limit(10_u8)
            .offset(10_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn test_required_params_endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = Ordinal::builder()
            .congress(117_u8)
            .nomination_number(2467_u32)
            .ordinal(1_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }

    #[tokio::test]
    async fn test_format_endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = Ordinal::builder()
            .format(Format::Xml)
            .congress(117_u8)
            .nomination_number(2467_u32)
            .ordinal(1_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }

    #[tokio::test]
    async fn test_offset_endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = Ordinal::builder()
            .congress(117_u8)
            .nomination_number(2467_u32)
            .ordinal(1_u32)
            .offset(10_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }

    #[tokio::test]
    async fn test_limit_endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = Ordinal::builder()
            .congress(117_u8)
            .nomination_number(2467_u32)
            .ordinal(1_u32)
            .limit(10_u8)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }

    #[tokio::test]
    async fn test_offset_limit_endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = Ordinal::builder()
            .congress(117_u8)
            .nomination_number(2467_u32)
            .ordinal(1_u32)
            .offset(10_u32)
            .limit(10_u8)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
