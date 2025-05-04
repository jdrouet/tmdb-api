use crate::common::PaginatedResult;

pub type Params<'a> = crate::common::LanguagePageParams<'a>;

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieList {
    pub id: u64,
    pub name: String,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub description: Option<String>,
    pub list_type: String,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub poster_path: Option<String>,
    pub iso_639_1: String,
    pub item_count: u64,
    pub favorite_count: u64,
}

impl<E: crate::client::Executor> crate::Client<E> {
    /// Get the lists that a movie has been added to.
    ///
    /// ```rust
    /// use tmdb_api::client::Client;
    /// use tmdb_api::client::reqwest::ReqwestExecutor;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
    ///     match client.get_movie_lists(42, &Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn get_movie_lists(
        &self,
        movie_id: u64,
        params: &Params<'_>,
    ) -> crate::Result<PaginatedResult<MovieList>> {
        let url = format!("/movie/{movie_id}/lists");
        self.execute(&url, params).await
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::client::Client;
    use crate::client::reqwest::ReqwestExecutor;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/movie/550/lists")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-lists.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let result = client
            .get_movie_lists(550, &Default::default())
            .await
            .unwrap();
        assert_eq!(result.page, 1);
        assert_eq!(result.results.len(), 20);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/movie/550/lists")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let err = client
            .get_movie_lists(550, &Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/movie/550/lists")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let err = client
            .get_movie_lists(550, &Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::client::Client;
    use crate::client::reqwest::ReqwestExecutor;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);
        let result = client
            .get_movie_lists(550, &Default::default())
            .await
            .unwrap();
        assert_eq!(result.page, 1);
    }
}
