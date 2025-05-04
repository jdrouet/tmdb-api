//! https://developer.themoviedb.org/reference/certification-movie-list
//! https://developer.themoviedb.org/reference/certifications-tv-list

use std::collections::HashMap;

use crate::client::Executor;

use super::Certification;

const TV_PATH: &str = "/certification/tv/list";
const MOVIE_PATH: &str = "/certification/movie/list";

#[derive(Serialize, Deserialize)]
struct CertificationResult {
    certifications: HashMap<String, Vec<Certification>>,
}

impl<E: Executor> crate::Client<E> {
    /// Get an up to date list of the officially supported movie certifications on TMDB
    ///
    /// ```rust
    /// use tmdb_api::Client;
    /// use tmdb_api::client::reqwest::ReqwestExecutor;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
    ///     match client.list_movie_certifications().await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn list_movie_certifications(
        &self,
    ) -> Result<HashMap<String, Vec<Certification>>, crate::error::Error> {
        self.execute::<CertificationResult, _>(MOVIE_PATH, &())
            .await
            .map(|res| res.certifications)
    }

    /// Get an up to date list of the officially supported tv show certifications on TMDB
    ///
    /// ```rust
    /// use tmdb_api::Client;
    /// use tmdb_api::client::reqwest::ReqwestExecutor;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
    ///     match client.list_tvshow_certifications().await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn list_tvshow_certifications(
        &self,
    ) -> Result<HashMap<String, Vec<Certification>>, crate::error::Error> {
        self.execute::<CertificationResult, _>(TV_PATH, &())
            .await
            .map(|res| res.certifications)
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::Client;
    use crate::client::reqwest::ReqwestExecutor;

    #[tokio::test]
    async fn tv_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let m = server
            .mock("GET", super::TV_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/certification-tv-list.json"))
            .create_async()
            .await;
        let result = client.list_tvshow_certifications().await.unwrap();
        assert!(!result.is_empty());
        m.assert_async().await;
    }

    #[tokio::test]
    async fn movie_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let m = server
            .mock("GET", super::MOVIE_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/certification-movie-list.json"))
            .create_async()
            .await;
        let result = client.list_movie_certifications().await.unwrap();
        assert!(!result.is_empty());
        m.assert_async().await;
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", super::TV_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = client.list_tvshow_certifications().await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", super::TV_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = client.list_tvshow_certifications().await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::client::Client;
    use crate::client::reqwest::ReqwestExecutor;

    #[tokio::test]
    async fn execute_tv() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);
        let result = client.list_tvshow_certifications().await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn execute_movie() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);
        let result = client.list_movie_certifications().await.unwrap();
        assert!(!result.is_empty());
    }
}
