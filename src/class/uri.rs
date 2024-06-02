use crate::util::invalid_argument_exception;
use ext_php_rs::prelude::*;
use http::uri::{Authority, PathAndQuery, Scheme};

#[php_class(name = "Takaram\\Psr7\\Internal\\Uri")]
pub struct Uri {
    scheme: Option<Scheme>,
    authority: Option<Authority>,
    path_and_query: Option<PathAndQuery>,
    fragment: Option<String>,
}

impl Uri {
    pub fn new<S: Into<String>>(str: S) -> Result<Self, String> {
        let str = str.into();
        str.parse::<http::Uri>()
            .map_err(|_| format!("Failed to parse URI: {str}", str = str.clone()))
            .map(|uri| {
                let parts = uri.into_parts();
                Self {
                    scheme: parts.scheme,
                    authority: parts.authority,
                    path_and_query: parts.path_and_query,
                    fragment: str.find('#').map(|pos| String::from(&str[(pos + 1)..])),
                }
            })
    }
}

#[php_impl]
impl Uri {
    pub fn __construct(str: String) -> PhpResult<Self> {
        Uri::new(str).map_err(|err| PhpException::new(err, 0, invalid_argument_exception()))
    }

    pub fn get_scheme(&self) -> String {
        self.scheme
            .as_ref()
            .map_or_else(|| "".to_string(), |scheme| scheme.as_str().to_lowercase())
    }

    pub fn get_authority(&self) -> String {
        self.authority
            .as_ref()
            .map(Authority::as_str)
            .unwrap_or("")
            .to_string()
    }

    pub fn get_user_info(&self) -> String {
        let Some(authority) = self.authority.as_ref().map(Authority::as_str) else {
            return "".to_string();
        };
        let Some(at_pos) = authority.find('@') else {
            return "".to_string();
        };
        String::from(&authority[..at_pos])
    }

    pub fn get_host(&self) -> String {
        self.authority
            .as_ref()
            .map_or("", Authority::host)
            .to_string()
    }

    pub fn get_port(&self) -> Option<u16> {
        self.authority
            .as_ref()
            .and_then(Authority::port_u16)
            .or_else(|| match self.scheme.as_ref().map(|s| s.as_str())? {
                "http" => Some(80),
                "https" => Some(443),
                _ => None,
            })
    }

    pub fn get_path(&self) -> String {
        self.path_and_query
            .as_ref()
            .map_or("", PathAndQuery::path)
            .to_string()
    }

    pub fn get_query(&self) -> String {
        self.path_and_query
            .as_ref()
            .and_then(PathAndQuery::query)
            .unwrap_or("")
            .to_string()
    }

    pub fn get_fragment(&self) -> String {
        self.fragment.clone().unwrap_or_else(|| "".to_string())
    }

    #[rename("__toString")]
    pub fn to_string(&self) -> String {
        let scheme = self.scheme.as_ref().map(Scheme::as_str);
        let authority = self.authority.as_ref().map(Authority::as_str);
        let path_and_query = self.path_and_query.as_ref().map(PathAndQuery::as_str);

        let mut result = scheme.map_or_else(
            || {
                format!(
                    "{}{}",
                    authority.unwrap_or(""),
                    path_and_query.unwrap_or("")
                )
            },
            |scheme| {
                format!(
                    "{}://{}{}",
                    scheme,
                    authority.unwrap_or(""),
                    path_and_query.unwrap_or("")
                )
            },
        );
        if let Some(frag) = &self.fragment {
            result.push('#');
            result.push_str(&frag);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_scheme_exist() {
        let uri = Uri::new("http://example.com/").unwrap();
        assert_eq!(uri.get_scheme(), "http");
    }

    #[test]
    fn get_scheme_not_exist() {
        let uri = Uri::new("/path").unwrap();
        assert_eq!(uri.get_scheme(), "");
    }

    #[test]
    fn get_authority_exist() {
        let uri = Uri::new("http://user:pass@example.com:8080/").unwrap();
        assert_eq!(uri.get_authority(), "user:pass@example.com:8080");
    }

    #[test]
    fn get_authority_not_exist() {
        let uri = Uri::new("/path").unwrap();
        assert_eq!(uri.get_authority(), "");
    }

    #[test]
    fn get_user_info_exist() {
        let uri = Uri::new("http://user:pass@example.com/path").unwrap();
        assert_eq!(uri.get_user_info(), "user:pass");
    }

    #[test]
    fn get_user_info_no_authority() {
        let uri = Uri::new("/path").unwrap();
        assert_eq!(uri.get_user_info(), "");
    }

    #[test]
    fn get_user_info_host_only() {
        let uri = Uri::new("http://example.com/path").unwrap();
        assert_eq!(uri.get_user_info(), "");
    }

    #[test]
    fn get_host_exist() {
        let uri = Uri::new("http://user:pass@example.com:8080/").unwrap();
        assert_eq!(uri.get_host(), "example.com");
    }

    #[test]
    fn get_host_not_exist() {
        let uri = Uri::new("/path").unwrap();
        assert_eq!(uri.get_host(), "");
    }

    #[test]
    fn get_port_exist() {
        let uri = Uri::new("http://user:pass@example.com:8080/").unwrap();
        assert_eq!(uri.get_port(), Some(8080));
    }

    #[test]
    fn get_port_implicit_http() {
        let uri = Uri::new("http://example.com/").unwrap();
        assert_eq!(uri.get_port(), Some(80));
    }

    #[test]
    fn get_port_implicit_https() {
        let uri = Uri::new("https://example.com/").unwrap();
        assert_eq!(uri.get_port(), Some(443));
    }

    #[test]
    fn get_port_not_exist() {
        let uri = Uri::new("/path").unwrap();
        assert_eq!(uri.get_port(), None);
    }

    #[ignore]
    #[test]
    fn get_path_empty() {
        let uri = Uri::new("http://example.com").unwrap();
        assert_eq!(uri.get_path(), "");
    }

    #[test]
    fn get_path_absolute() {
        let uri = Uri::new("http://example.com/path").unwrap();
        assert_eq!(uri.get_path(), "/path");
    }

    #[test]
    fn get_path_with_query() {
        let uri = Uri::new("http://example.com/path?foo=bar").unwrap();
        assert_eq!(uri.get_path(), "/path");
    }

    #[test]
    fn get_path_with_fragment() {
        let uri = Uri::new("http://example.com/path#foo").unwrap();
        assert_eq!(uri.get_path(), "/path");
    }

    #[ignore]
    #[test]
    fn get_path_rootless() {
        let uri = Uri::new("foo/bar").unwrap();
        assert_eq!(uri.get_path(), "foo/bar");
    }

    #[test]
    fn get_path_percent_encoded() {
        let uri = Uri::new("/foo%2Fbar").unwrap();
        assert_eq!(uri.get_path(), "/foo%2Fbar");
    }

    #[test]
    fn get_query_exist() {
        let uri = Uri::new("/path?foo=bar&baz=qux").unwrap();
        assert_eq!(uri.get_query(), "foo=bar&baz=qux");
    }

    #[test]
    fn get_query_with_fragment() {
        let uri = Uri::new("/path?foo=bar&baz=qux#foo").unwrap();
        assert_eq!(uri.get_query(), "foo=bar&baz=qux");
    }

    #[test]
    fn get_query_not_exist() {
        let uri = Uri::new("/path").unwrap();
        assert_eq!(uri.get_query(), "");
    }

    #[test]
    fn get_fragment_exist() {
        let uri = Uri::new("/path#foo").unwrap();
        assert_eq!(uri.get_fragment(), "foo");
    }

    #[test]
    fn get_fragment_multiple_hash() {
        let uri = Uri::new("/path#foo#bar").unwrap();
        assert_eq!(uri.get_fragment(), "foo#bar");
    }

    #[test]
    fn get_fragment_not_exist() {
        let uri = Uri::new("/path").unwrap();
        assert_eq!(uri.get_fragment(), "");
    }

    #[test]
    fn to_string() {
        let uri = Uri::new("http://user:pass@example.com:8080/").unwrap();
        assert_eq!(uri.to_string(), "http://user:pass@example.com:8080/");

        let uri = Uri::new("https://user:pass@example.com/path?foo=bar#baz").unwrap();
        assert_eq!(
            uri.to_string(),
            "https://user:pass@example.com/path?foo=bar#baz"
        );

        let uri = Uri::new("/path#baz").unwrap();
        assert_eq!(uri.to_string(), "/path#baz");
    }
}
