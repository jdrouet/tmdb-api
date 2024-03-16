use std::borrow::Cow;

use chrono::NaiveDate;

const TV_PATH: &str = "/tv/changes";
const MOVIE_PATH: &str = "/movie/changes";
const PERSON_PATH: &str = "/person/changes";

/// Command to list changes
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::changes::list::ChangeList;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = ChangeList::tv();
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct ChangeList {
    path: &'static str,
    /// Filter the results with a start date.
    pub start_date: Option<NaiveDate>,
    /// Filter the results with an end date.
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
            res.push(("start_date", Cow::Owned(start_date.to_string())));
        }
        if let Some(ref end_date) = self.end_date {
            res.push(("end_date", Cow::Owned(end_date.to_string())));
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
    use chrono::NaiveDate;
    use mockito::Matcher;

    use crate::Client;
    use crate::prelude::Command;

    use super::ChangeList;

    #[tokio::test]
    async fn tv_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let cmd = ChangeList::tv();

        let _m = server
            .mock("GET", super::TV_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/tv-all-changes.json"))
            .create_async()
            .await;
        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
    }

    #[tokio::test]
    async fn tv_works_with_args() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", super::TV_PATH)
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
            .create_async()
            .await;

        let result = ChangeList::tv()
            .with_start_date(Some(NaiveDate::from_ymd_opt(2015, 3, 14).unwrap()))
            .with_end_date(Some(NaiveDate::from_ymd_opt(2019, 3, 14).unwrap()))
            .with_page(Some(2))
            .execute(&client)
            .await
            .unwrap();
        assert_eq!(result.page, 1);
    }

    #[tokio::test]
    async fn movie_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let cmd = ChangeList::movie();

        let _m = server
            .mock("GET", super::MOVIE_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-all-changes.json"))
            .create_async()
            .await;
        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
    }

    #[tokio::test]
    async fn person_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let cmd = ChangeList::person();

        let _m = server
            .mock("GET", super::PERSON_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-all-changes.json"))
            .create_async()
            .await;
        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let cmd = ChangeList::tv();

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
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let cmd = ChangeList::tv();

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
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::Client;
    use crate::prelude::Command;

    use super::ChangeList;

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
