//! https://developer.themoviedb.org/reference/configuration-countries

use std::borrow::Cow;

/// Get a list of all countries
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::client::reqwest::ReqwestExecutor;
/// use tmdb_api::configuration::countries::Countries;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
///     let result = Countries::default().execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {res:#?}"),
///         Err(err) => eprintln!("error: {err:?}"),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct Countries {
    language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CountriesResult {
    pub iso_3166_1: String,
    pub english_name: String,
    pub native_name: String,
}

impl Countries {
    pub fn new() -> Self {
        Self { language: None }
    }

    pub fn with_language(mut self, value: Option<String>) -> Self {
        self.language = value;
        self
    }
}

impl crate::prelude::Command for Countries {
    type Output = Vec<CountriesResult>;

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed("/configuration/countries")
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        if let Some(ref language) = self.language {
            vec![("language", Cow::Borrowed(language))]
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::client::reqwest::ReqwestExecutor;
    use crate::prelude::Command;
    use crate::Client;

    use super::Countries;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/configuration/countries")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/configuration-countries.json"))
            .create_async()
            .await;

        let result = Countries::default().execute(&client).await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/configuration/countries")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = Countries::default().execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/configuration/countries")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = Countries::default().execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::client::reqwest::ReqwestExecutor;
    use crate::prelude::Command;
    use crate::Client;

    use super::Countries;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);

        let result = Countries::default().execute(&client).await.unwrap();
        assert!(!result.is_empty());
    }
}
