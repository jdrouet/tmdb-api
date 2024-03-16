//! https://developer.themoviedb.org/reference/tv-series-content-ratings

use std::borrow::Cow;

#[derive(Clone, Debug, Default)]
pub(crate) struct TVShowContentRating {
    pub(crate) id: u64,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct ContentRatingResult {
    pub(crate) id: u64,
    pub(crate) results: Vec<ContentRatingBase>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct ContentRatingBase {
    pub(crate) descriptors: Vec<String>,
    pub(crate) iso_3166_1: String,
    pub(crate) rating: String,
}

impl TVShowContentRating {
    pub(crate) fn new(tv_show_id: u64) -> Self {
        Self { id: tv_show_id }
    }
}

impl crate::prelude::Command for TVShowContentRating {
    type Output = ContentRatingResult;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/tv/{}/content_ratings", self.id))
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::prelude::Command;
    use crate::Client;

    use super::TVShowContentRating;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/1399/content_ratings")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/tv-content-ratings.json"))
            .create_async()
            .await;

        let result = TVShowContentRating::new(1399)
            .execute(&client)
            .await
            .unwrap();
        assert_eq!(result.id, 1399);
        assert!(!result.results.is_empty());
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
            .mock("GET", "/tv/1399/content_ratings")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = TVShowContentRating::new(1399)
            .execute(&client)
            .await
            .unwrap_err();
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
            .mock("GET", "/tv/1399/content_ratings")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = TVShowContentRating::new(1399)
            .execute(&client)
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::prelude::Command;
    use crate::Client;

    use super::TVShowContentRating;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        let result = TVShowContentRating::new(1399)
            .execute(&client)
            .await
            .unwrap();
        assert_eq!(result.id, 1399);
        assert!(!result.results.is_empty());
    }
}
