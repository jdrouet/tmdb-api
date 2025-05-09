use crate::client::Executor;

#[derive(Debug, Deserialize)]
pub struct MovieExternalIds {
    pub id: u64,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub imdb_id: Option<String>,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub wikidata_id: Option<String>,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub facebook_id: Option<String>,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub instagram_id: Option<String>,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub twitter_id: Option<String>,
}

impl<E: Executor> crate::Client<E> {
    /// Get movie external ids
    ///
    /// ```rust
    /// use tmdb_api::client::Client;
    /// use tmdb_api::client::reqwest::Client as ReqwestClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestClient>::new("this-is-my-secret-token".into());
    ///     match client.get_movie_external_ids(42).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn get_movie_external_ids(&self, movie_id: u64) -> crate::Result<MovieExternalIds> {
        let url = format!("/movie/{movie_id}/external_ids");
        self.execute(&url, &()).await
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
        let m = server
            .mock("GET", "/movie/335984/external_ids")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-external-ids.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let result = client.get_movie_external_ids(335984).await.unwrap();
        assert_eq!(result.id, 550);

        m.assert_async().await;
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", "/movie/42/external_ids")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let err = client.get_movie_external_ids(42).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 7);

        m.assert_async().await;
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", "/movie/42/external_ids")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let err = client.get_movie_external_ids(42).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);

        m.assert_async().await;
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
        let result = client.get_movie_external_ids(335984).await.unwrap();
        assert_eq!(result.id, 335984);
    }
}
