use crate::client::Executor;

pub use crate::changes::list::Params;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MovieChange {
    pub key: String,
    pub items: Vec<MovieChangeItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MovieChangeItem {
    pub id: String,
    pub action: String,
    pub time: chrono::DateTime<chrono::Utc>,
    pub iso_639_1: String,
    pub iso_3166_1: String,
    // TODO handle really dynamic kind of values
    // pub value: String,
    // pub original_value: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    pub changes: Vec<MovieChange>,
}

impl<E: Executor> crate::Client<E> {
    /// List changes for a movie
    ///
    /// ```rust
    /// use tmdb_api::client::Client;
    /// use tmdb_api::client::reqwest::Client as ReqwestClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestClient>::new("this-is-my-secret-token".into());
    ///     match client.get_movie_changes(42, &Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn get_movie_changes(
        &self,
        movie_id: u64,
        params: &Params,
    ) -> crate::Result<Response> {
        let url = format!("/movie/{movie_id}/changes");
        self.execute(&url, params).await
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
            .mock("GET", "/movie/3/changes")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-single-changes.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let result = client
            .get_movie_changes(3, &Default::default())
            .await
            .unwrap();
        assert_eq!(result.changes.len(), 1);

        m.assert_async().await;
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", "/movie/1/changes")
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
        let err = client
            .get_movie_changes(1, &Default::default())
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
            .mock("GET", "/movie/1/changes")
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
        let err = client
            .get_movie_changes(1, &Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);

        m.assert_async().await;
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::Params;
    use crate::client::Client;
    use crate::client::reqwest::Client as ReqwestClient;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestClient>::new(secret);
        let params = Params::default()
            .with_start_date(chrono::NaiveDate::from_ymd_opt(2015, 3, 14).unwrap())
            .with_end_date(chrono::NaiveDate::from_ymd_opt(2019, 3, 14).unwrap());
        let result = client.get_movie_changes(1, &params).await.unwrap();
        assert!(result.changes.is_empty());
    }
}
