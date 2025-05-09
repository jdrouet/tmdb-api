//! https://developer.themoviedb.org/reference/configuration-jobs

use crate::client::Executor;

#[derive(Debug, Deserialize)]
pub struct Job {
    pub department: String,
    pub jobs: Vec<String>,
}

impl<E: Executor> crate::Client<E> {
    /// Get a list of all jobs
    ///
    /// ```rust
    /// use tmdb_api::client::Client;
    /// use tmdb_api::client::reqwest::Client as ReqwestClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestClient>::new("this-is-my-secret-token".into());
    ///     match client.list_jobs().await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn list_jobs(&self) -> crate::Result<Vec<Job>> {
        self.execute::<Vec<Job>, _>("/configuration/jobs", &())
            .await
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::Client;
    use crate::client::reqwest::Client as ReqwestClient;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", "/configuration/jobs")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/configuration-jobs.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let result = client.list_jobs().await.unwrap();
        assert!(!result.is_empty());
        m.assert_async().await;
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", "/configuration/jobs")
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
        let err = client.list_jobs().await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 7);
        m.assert_async().await;
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/configuration/jobs")
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
        let err = client.list_jobs().await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::Client;
    use crate::client::reqwest::Client as ReqwestClient;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestClient>::new(secret);
        let result = client.list_jobs().await.unwrap();
        assert!(!result.is_empty());
    }
}
