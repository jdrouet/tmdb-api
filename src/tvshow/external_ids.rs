//! https://developer.themoviedb.org/reference/tv-series-external-ids

use std::borrow::Cow;

use crate::common::external_ids::TVShowExternalIdsResult;

/// Get the external ids of a tvshow.
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::tvshow::external_ids::TVShowExternalIds;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = TVShowExternalIds::new(1399);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct TVShowExternalIds {
    pub id: u64,
}

impl TVShowExternalIds {
    pub fn new(tv_show_id: u64) -> Self {
        Self { id: tv_show_id }
    }
}

impl crate::prelude::Command for TVShowExternalIds {
    type Output = TVShowExternalIdsResult;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/tv/{}/external_ids", self.id))
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

    use super::TVShowExternalIds;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/1399/external_ids")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/tv-external-ids.json"))
            .create_async()
            .await;

        let cmd = TVShowExternalIds::new(1399);
        let result = cmd.execute(&client).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn complex_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/1399/external_ids")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/tv-external-ids.json"))
            .create_async()
            .await;

        let result = TVShowExternalIds::new(1399).execute(&client).await.unwrap();
        assert_eq!(result.id, 1399);
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
            .mock("GET", "/tv/1399/external_ids")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = TVShowExternalIds::new(1399)
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
            .mock("GET", "/tv/0/external_ids")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = TVShowExternalIds::new(0)
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

    use super::TVShowExternalIds;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let cmd = TVShowExternalIds::new(1399);

        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.id, 1399);
    }
}
