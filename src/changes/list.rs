use std::borrow::Cow;

const TV_PATH: &str = "/tv/changes";
const MOVIE_PATH: &str = "/movie/changes";
const PERSON_PATH: &str = "/person/changes";

/// Command to list changes
#[derive(Clone, Debug, Default)]
pub struct ChangeList {
    path: &'static str,
}

impl ChangeList {
    pub fn tv() -> Self {
        Self { path: TV_PATH }
    }

    pub fn movie() -> Self {
        Self { path: MOVIE_PATH }
    }

    pub fn person() -> Self {
        Self { path: PERSON_PATH }
    }
}

#[async_trait::async_trait]
impl crate::prelude::Command for ChangeList {
    type Output = crate::common::PaginatedResult<super::Change>;

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed(self.path)
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        Vec::new()
    }

    async fn execute(&self, client: &crate::Client) -> Result<Self::Output, crate::error::Error> {
        client.execute(self.path().as_ref(), self.params()).await
    }
}

#[cfg(test)]
mod tests {
    use super::ChangeList;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn tv_works() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = ChangeList::tv();

        let _m = mock("GET", super::TV_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/change-list-success.json"))
            .create();
        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
    }

    #[tokio::test]
    async fn movie_works() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = ChangeList::movie();

        let _m = mock("GET", super::MOVIE_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/change-list-success.json"))
            .create();
        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
    }

    #[tokio::test]
    async fn person_works() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = ChangeList::person();

        let _m = mock("GET", super::PERSON_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/change-list-success.json"))
            .create();
        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = ChangeList::tv();

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
        let cmd = ChangeList::tv();

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
    use super::ChangeList;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn execute_tv() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        let result = ChangeList::tv().execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
    }

    #[tokio::test]
    async fn execute_movie() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        let result = ChangeList::movie().execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
    }

    #[tokio::test]
    async fn execute_person() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        let result = ChangeList::person().execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
    }
}
