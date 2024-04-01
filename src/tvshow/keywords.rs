//! https://developer.themoviedb.org/reference/tv-series-keywords

use std::borrow::Cow;

use crate::common::keyword::Keyword;

/// Command to get the keywords of a tvshow.
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::client::reqwest::ReqwestExecutor;
/// use tmdb_api::tvshow::keywords::TVShowKeywords;
///
/// #[tokio::main]
/// async fn main() {
/// 	let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
///     let cmd = TVShowKeywords::new(1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct TVShowKeywords {
    pub tv_show_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct TVShowKeywordsResult {
    pub id: u64,
    pub results: Vec<Keyword>,
}

impl TVShowKeywords {
    pub fn new(tv_show_id: u64) -> Self {
        Self { tv_show_id }
    }
}

impl crate::prelude::Command for TVShowKeywords {
    type Output = TVShowKeywordsResult;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/tv/{}/keywords", self.tv_show_id))
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use super::TVShowKeywords;
    use crate::client::reqwest::ReqwestExecutor;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/1399/keywords")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/tv-keywords.json"))
            .create_async()
            .await;

        let result = TVShowKeywords::new(1399).execute(&client).await.unwrap();
        assert_eq!(result.id, 1399);
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
            .mock("GET", "/tv/1399/keywords")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = TVShowKeywords::new(1399)
            .execute(&client)
            .await
            .unwrap_err();
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
            .mock("GET", "/tv/1399/keywords")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = TVShowKeywords::new(1399)
            .execute(&client)
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::client::reqwest::ReqwestExecutor;
    use crate::prelude::Command;
    use crate::Client;

    use super::TVShowKeywords;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);

        let result = TVShowKeywords::new(1399).execute(&client).await.unwrap();
        assert_eq!(result.id, 1399);
    }
}
