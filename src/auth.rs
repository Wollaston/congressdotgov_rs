use url::Url;

#[derive(Debug, Clone)]
pub enum Auth {
    Token(String),
}
