use crate::common::PaginatedResult;

pub type Params<'a> = crate::common::LanguagePageParams<'a>;

impl<E: crate::client::Executor> crate::Client<E> {
    /// Get a list of the current popular tvshows on TMDB. This list updates daily.
    ///
    /// ```rust
    /// use tmdb_api::client::Client;
    /// use tmdb_api::client::reqwest::Client as ReqwestClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestClient>::new("this-is-my-secret-token".into());
    ///     match client.list_popular_movies(&Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn list_popular_tvshows(
        &self,
        params: &Params<'_>,
    ) -> crate::Result<PaginatedResult<super::TVShowShort>> {
        self.execute("/tv/popular", params).await
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::client::reqwest::Client as ReqwestClient;
    use mockito::Matcher;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/popular")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/tv-popular.json"))
            .create_async()
            .await;

        let result = client
            .list_popular_tvshows(&Default::default())
            .await
            .unwrap();
        assert_eq!(result.page, 1);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/popular")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = client
            .list_popular_tvshows(&Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/popular")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = client
            .list_popular_tvshows(&Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::client::Client;
    use crate::client::reqwest::Client as ReqwestClient;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestClient>::new(secret);

        let _result = client
            .list_popular_tvshows(&Default::default())
            .await
            .unwrap();
    }
}
