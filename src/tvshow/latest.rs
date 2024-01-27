use std::borrow::Cow;

/// Get the most newly created show. This is a live response and will continuously change.
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::tvshow::latest::TVShowLatest;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let result = TVShowLatest::default().execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct TVShowLatest {
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
}

impl TVShowLatest {
    pub fn with_language(mut self, value: Option<String>) -> Self {
        self.language = value;
        self
    }
}

impl crate::prelude::Command for TVShowLatest {
    type Output = super::TVShow;

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed("/tv/latest")
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        let mut res = Vec::new();
        if let Some(ref language) = self.language {
            res.push(("language", Cow::Borrowed(language.as_str())))
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::TVShowLatest;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::Matcher;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/latest")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/tv-latest.json"))
            .create_async()
            .await;

        let result = TVShowLatest::default().execute(&client).await.unwrap();
        assert_eq!(result.inner.id, 1399);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/latest")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = TVShowLatest::default().execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/latest")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = TVShowLatest::default().execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::TVShowLatest;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    #[ignore = "The API doesn't work as expected. For history: https://www.themoviedb.org/talk/65b3e54e5541fa0164b18674"]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        // This route is currently down, fix the test accordingly to pass them
        let _result = TVShowLatest::default().execute(&client).await.unwrap_err();
    }
}
