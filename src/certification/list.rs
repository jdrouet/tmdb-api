//! https://developer.themoviedb.org/reference/certification-movie-list
//! https://developer.themoviedb.org/reference/certifications-tv-list

use std::borrow::Cow;
use std::collections::HashMap;

use crate::client::Executor;

use super::Certification;

const TV_PATH: &str = "/certification/tv/list";
const MOVIE_PATH: &str = "/certification/movie/list";

#[derive(Serialize, Deserialize)]
struct CertificationResult {
    certifications: HashMap<String, Vec<Certification>>,
}

/// Command to list certifications
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::client::reqwest::ReqwestExecutor;
/// use tmdb_api::certification::list::CertificationList;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
///     let cmd = CertificationList::tv();
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct CertificationList {
    path: &'static str,
}

impl CertificationList {
    pub fn tv() -> Self {
        Self { path: TV_PATH }
    }

    pub fn movie() -> Self {
        Self { path: MOVIE_PATH }
    }
}

impl crate::prelude::Command for CertificationList {
    type Output = HashMap<String, Vec<Certification>>;

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed(self.path)
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        Vec::new()
    }

    async fn execute<E: Executor>(
        &self,
        client: &crate::Client<E>,
    ) -> Result<Self::Output, crate::error::Error> {
        client
            .execute::<CertificationResult>(self.path().as_ref(), self.params())
            .await
            .map(|res| res.certifications)
    }
}

impl<E: Executor> crate::Client<E> {
    /// Get an up to date list of the officially supported movie certifications on TMDB
    ///
    /// ```rust
    /// use tmdb_api::Client;
    /// use tmdb_api::client::reqwest::ReqwestExecutor;
    /// use tmdb_api::certification::list::CertificationList;
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
        self.execute::<CertificationResult>(MOVIE_PATH, Vec::default())
            .await
            .map(|res| res.certifications)
    }

    /// Get an up to date list of the officially supported tv show certifications on TMDB
    ///
    /// ```rust
    /// use tmdb_api::Client;
    /// use tmdb_api::client::reqwest::ReqwestExecutor;
    /// use tmdb_api::certification::list::CertificationList;
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
        self.execute::<CertificationResult>(TV_PATH, Vec::default())
            .await
            .map(|res| res.certifications)
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::Client;
    use crate::client::reqwest::ReqwestExecutor;
    use crate::prelude::Command;

    use super::CertificationList;

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
            .expect(2)
            .create_async()
            .await;
        let result = CertificationList::tv().execute(&client).await.unwrap();
        assert!(!result.is_empty());
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
            .expect(2)
            .create_async()
            .await;
        let result = CertificationList::movie().execute(&client).await.unwrap();
        assert!(!result.is_empty());
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
        let cmd = CertificationList::tv();

        let _m = server
            .mock("GET", super::TV_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;
        let err = cmd.execute(&client).await.unwrap_err();
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
        let cmd = CertificationList::tv();

        let _m = server
            .mock("GET", super::TV_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;
        let err = cmd.execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::client::Client;
    use crate::client::reqwest::ReqwestExecutor;
    use crate::prelude::Command;

    use super::CertificationList;

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
