use std::borrow::Cow;

use crate::client::Executor;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Params<'a> {
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<Cow<'a, str>>,
}

impl<'a> Params<'a> {
    pub fn set_language(&mut self, value: impl Into<Cow<'a, str>>) {
        self.language = Some(value.into());
    }

    pub fn with_language(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.set_language(value);
        self
    }
}

impl<E: Executor> crate::Client<E> {
    /// Get movie details
    ///
    /// ```rust
    /// use tmdb_api::client::Client;
    /// use tmdb_api::client::reqwest::ReqwestExecutor;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
    ///     match client.get_movie_details(42, &Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn get_movie_details(
        &self,
        movie_id: u64,
        params: &Params<'_>,
    ) -> crate::Result<super::Movie> {
        let url = format!("/movie/{movie_id}");
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
        let m = server
            .mock("GET", "/movie/550")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-details.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let result = client
            .get_movie_details(550, &Default::default())
            .await
            .unwrap();
        assert_eq!(result.inner.id, 550);

        m.assert_async().await;
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", "/movie/550")
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
            .get_movie_details(550, &Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 7);

        m.assert_async().await;
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", "/movie/550")
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
            .get_movie_details(550, &Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);

        m.assert_async().await;
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

        for i in [2, 3, 5, 6, 8, 9, 11] {
            let result = client
                .get_movie_details(i, &Default::default())
                .await
                .unwrap();
            assert_eq!(result.inner.id, i);
        }
    }
}
