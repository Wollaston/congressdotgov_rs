/// The different ways to authenticate with the congress.gov
/// API. Currently, only token-based authentication exists.
#[derive(Debug, Clone)]
pub enum Auth {
    Token(String),
}
