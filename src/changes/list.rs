use crate::util::date::format_date;
use chrono::NaiveDate;
use std::borrow::Cow;

const TV_PATH: &str = "/tv/changes";
const MOVIE_PATH: &str = "/movie/changes";
const PERSON_PATH: &str = "/person/changes";

/// Command to list changes
#[derive(Clone, Debug, Default)]
pub struct ChangeList {
    path: &'static str,
    /// Filter the results with a start date.
    pub start_date: Option<NaiveDate>,
    /// Filter the results with a end date.
    pub end_date: Option<NaiveDate>,
    /// Which page to query.
    pub page: Option<u32>,
}

impl ChangeList {
    pub fn tv() -> Self {
        Self {
            path: TV_PATH,
            start_date: None,
            end_date: None,
            page: None,
        }
    }

    pub fn movie() -> Self {
        Self {
            path: MOVIE_PATH,
            start_date: None,
            end_date: None,
            page: None,
        }
    }

    pub fn person() -> Self {
        Self {
            path: PERSON_PATH,
            start_date: None,
            end_date: None,
            page: None,
        }
    }

    pub fn with_start_date(mut self, value: Option<NaiveDate>) -> Self {
        self.start_date = value;
        self
    }

    pub fn with_end_date(mut self, value: Option<NaiveDate>) -> Self {
        self.end_date = value;
        self
    }

    pub fn with_page(mut self, value: Option<u32>) -> Self {
        self.page = value;
        self
    }
}

#[async_trait::async_trait]
impl crate::prelude::Command for ChangeList {
    type Output = crate::common::PaginatedResult<super::Change>;

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed(self.path)
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        let mut res = Vec::with_capacity(3);
        if let Some(ref start_date) = self.start_date {
            res.push(("start_date", Cow::Owned(format_date(start_date))));
        }
        if let Some(ref end_date) = self.end_date {
            res.push(("end_date", Cow::Owned(format_date(end_date))));
        }
        if let Some(page) = self.page {
            res.push(("page", Cow::Owned(page.to_string())));
        }
        res
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
    use chrono::NaiveDate;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn tv_works() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = ChangeList::tv();

        let _m = mock("GET", super::TV_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/tv-all-changes.json"))
            .create();
        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
    }

    #[tokio::test]
    async fn tv_works_with_args() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());

        let _m = mock("GET", super::TV_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("start_date".into(), "2015-03-14".into()),
                Matcher::UrlEncoded("end_date".into(), "2019-03-14".into()),
                Matcher::UrlEncoded("page".into(), "2".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/tv-all-changes.json"))
            .create();

        let result = ChangeList::tv()
            .with_start_date(Some(NaiveDate::from_ymd(2015, 3, 14)))
            .with_end_date(Some(NaiveDate::from_ymd(2019, 3, 14)))
            .with_page(Some(2))
            .execute(&client)
            .await
            .unwrap();
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
            .with_body(include_str!("../../assets/movie-all-changes.json"))
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
            .with_body(include_str!("../../assets/movie-all-changes.json"))
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
