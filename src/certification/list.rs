use super::Certification;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

const TV_PATH: &str = "/certification/tv/list";
const MOVIE_PATH: &str = "/certification/movie/list";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CertificationResult {
    pub certifications: HashMap<String, Vec<Certification>>,
}

/// Command to list certifications
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

#[async_trait::async_trait]
impl crate::prelude::Command for CertificationList {
    type Output = HashMap<String, Vec<Certification>>;

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed(self.path)
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        Vec::new()
    }

    async fn execute(&self, client: &crate::Client) -> Result<Self::Output, crate::error::Error> {
        client
            .execute::<CertificationResult>(self.path().as_ref(), self.params())
            .await
            .map(|res| res.certifications)
    }
}

#[cfg(test)]
mod tests {
    use super::CertificationList;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn tv_works() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());

        let _m = mock("GET", super::TV_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!(
                "../../assets/certification-tv-list-success.json"
            ))
            .create();
        let result = CertificationList::tv().execute(&client).await.unwrap();
        assert_eq!(result.len(), 10);
    }

    #[tokio::test]
    async fn movie_works() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());

        let _m = mock("GET", super::MOVIE_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!(
                "../../assets/certification-movie-list-success.json"
            ))
            .create();
        let result = CertificationList::movie().execute(&client).await.unwrap();
        assert_eq!(result.len(), 15);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = CertificationList::tv();

        let _m = mock("GET", super::TV_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create();
        let err = cmd.execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = CertificationList::tv();

        let _m = mock("GET", super::TV_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create();
        let err = cmd.execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::CertificationList;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn execute_tv() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let result = CertificationList::tv().execute(&client).await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn execute_movie() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let result = CertificationList::movie().execute(&client).await.unwrap();
        assert!(!result.is_empty());
    }
}
