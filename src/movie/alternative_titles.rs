use std::borrow::Cow;

use crate::client::Executor;

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GetMovieAlternativeTitlesParams<'a> {
    /// The country to filter the alternative titles
    pub country: Option<Cow<'a, str>>,
}

impl<'a> GetMovieAlternativeTitlesParams<'a> {
    pub fn set_country(&mut self, value: impl Into<Cow<'a, str>>) {
        self.country = Some(value.into());
    }

    pub fn with_country(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.set_country(value);
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MovieAlternativeTitle {
    pub iso_3166_1: String,
    pub title: String,
    #[serde(
        deserialize_with = "crate::util::empty_string::deserialize",
        rename = "type"
    )]
    pub kind: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    pub titles: Vec<MovieAlternativeTitle>,
}

impl<E: Executor> crate::Client<E> {
    /// Command to get alternative titles for a movie
    ///
    /// ```rust
    /// use tmdb_api::client::Client;
    /// use tmdb_api::client::reqwest::ReqwestExecutor;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
    ///     match client.get_movie_alternative_titles(42, &Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn get_movie_alternative_titles(
        &self,
        movie_id: u64,
        params: &GetMovieAlternativeTitlesParams<'_>,
    ) -> crate::Result<Response> {
        let url = format!("/movie/{movie_id}/alternative_titles");
        self.execute(&url, params).await
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::client::reqwest::ReqwestExecutor;
    use mockito::Matcher;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", "/movie/3/alternative_titles")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-alternative-titles.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let result = client
            .get_movie_alternative_titles(3, &Default::default())
            .await
            .unwrap();
        assert!(!result.titles.is_empty());

        m.assert_async().await;
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", "/movie/1/alternative_titles")
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
            .get_movie_alternative_titles(1, &Default::default())
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
            .mock("GET", "/movie/1/alternative_titles")
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
            .get_movie_alternative_titles(1, &Default::default())
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
        let result = client
            .get_movie_alternative_titles(3, &Default::default())
            .await
            .unwrap();
        assert!(!result.titles.is_empty());
    }
}
