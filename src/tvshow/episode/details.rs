pub type Params<'a> = crate::common::LanguageParams<'a>;

impl<E: crate::client::Executor> crate::Client<E> {
    /// Get tvshow episode complete details
    ///
    /// ```rust
    /// use tmdb_api::client::Client;
    /// use tmdb_api::client::reqwest::Client as ReqwestClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestClient>::new("this-is-my-secret-token".into());
    ///     match client.get_tvshow_details(42, &Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn get_tvshow_episode_details(
        &self,
        tvshow_id: u64,
        season_number: u64,
        episode_number: u64,
        params: &Params<'_>,
    ) -> crate::Result<crate::tvshow::Episode> {
        let url = format!("/tv/{tvshow_id}/season/{season_number}/episode/{episode_number}");
        self.execute(&url, params).await
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::client::Client;
    use crate::client::reqwest::Client as ReqwestClient;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/1399/season/1/episode/1")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../../assets/tv-episode-details.json"))
            .create_async()
            .await;

        let result = client
            .get_tvshow_episode_details(1399, 1, 1, &Default::default())
            .await
            .unwrap();
        assert_eq!(result.inner.id, 63056);
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
            .mock("GET", "/tv/1399/season/1/episode/1")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = client
            .get_tvshow_episode_details(1399, 1, 1, &Default::default())
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
            .mock("GET", "/tv/1399/season/1/episode/1")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = client
            .get_tvshow_episode_details(1399, 1, 1, &Default::default())
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

        for tv_id in [1] {
            let _result = client
                .get_tvshow_episode_details(tv_id, 1, 1, &Default::default())
                .await
                .unwrap();
        }
    }
}
