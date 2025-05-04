use chrono::NaiveDate;

use crate::client::Executor;

const TV_PATH: &str = "/tv/changes";
const MOVIE_PATH: &str = "/movie/changes";
const PERSON_PATH: &str = "/person/changes";

#[derive(Clone, Debug, Default, Serialize)]
pub struct ChangeListParams {
    /// Filter the results with a start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    /// Filter the results with an end date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    /// Which page to query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl ChangeListParams {
    pub fn set_start_date(&mut self, value: NaiveDate) {
        self.start_date = Some(value);
    }

    pub fn with_start_date(mut self, value: NaiveDate) -> Self {
        self.set_start_date(value);
        self
    }

    pub fn set_end_date(&mut self, value: NaiveDate) {
        self.end_date = Some(value);
    }

    pub fn with_end_date(mut self, value: NaiveDate) -> Self {
        self.set_end_date(value);
        self
    }

    pub fn set_page(&mut self, value: u32) {
        self.page = Some(value);
    }

    pub fn with_page(mut self, value: u32) -> Self {
        self.set_page(value);
        self
    }
}

impl<E: Executor> crate::Client<E> {
    /// Get a list of all of the movie ids that have been changed in the past 24 hours.
    ///
    /// ```rust
    /// use tmdb_api::Client;
    /// use tmdb_api::client::reqwest::ReqwestExecutor;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
    ///     match client.list_movie_changes(&Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn list_movie_changes(
        &self,
        params: &ChangeListParams,
    ) -> crate::Result<crate::common::PaginatedResult<super::Change>> {
        self.execute(MOVIE_PATH, params).await
    }

    /// Get a list of all of the person ids that have been changed in the past 24 hours.
    ///
    /// ```rust
    /// use tmdb_api::Client;
    /// use tmdb_api::client::reqwest::ReqwestExecutor;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
    ///     match client.list_person_changes(&Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn list_person_changes(
        &self,
        params: &ChangeListParams,
    ) -> crate::Result<crate::common::PaginatedResult<super::Change>> {
        self.execute(PERSON_PATH, params).await
    }

    /// Get a list of all of the tvshow ids that have been changed in the past 24 hours.
    ///
    /// ```rust
    /// use tmdb_api::Client;
    /// use tmdb_api::client::reqwest::ReqwestExecutor;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
    ///     match client.list_tvshow_changes(&Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn list_tvshow_changes(
        &self,
        params: &ChangeListParams,
    ) -> crate::Result<crate::common::PaginatedResult<super::Change>> {
        self.execute(TV_PATH, params).await
    }
}

#[cfg(test)]
mod tests {
    use crate::changes::list::ChangeListParams;
    use crate::client::Client;
    use crate::client::reqwest::ReqwestExecutor;
    use chrono::NaiveDate;
    use mockito::Matcher;

    #[tokio::test]
    async fn tv_works() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", super::TV_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/tv-all-changes.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let result = client
            .list_tvshow_changes(&Default::default())
            .await
            .unwrap();
        assert_eq!(result.page, 1);

        m.assert_async().await;
    }

    #[tokio::test]
    async fn tv_works_with_args() {
        let mut server = mockito::Server::new_async().await;
        let m = server
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

        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let result = client
            .list_tvshow_changes(
                &ChangeListParams::default()
                    .with_start_date(NaiveDate::from_ymd_opt(2015, 3, 14).unwrap())
                    .with_end_date(NaiveDate::from_ymd_opt(2019, 3, 14).unwrap())
                    .with_page(2),
            )
            .await
            .unwrap();
        assert_eq!(result.page, 1);
        m.assert_async().await;
    }

    #[tokio::test]
    async fn movie_works() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", super::MOVIE_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-all-changes.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let result = client
            .list_movie_changes(&Default::default())
            .await
            .unwrap();
        assert_eq!(result.page, 1);

        m.assert_async().await;
    }

    #[tokio::test]
    async fn person_works() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", super::PERSON_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-all-changes.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let result = client
            .list_person_changes(&Default::default())
            .await
            .unwrap();
        assert_eq!(result.page, 1);

        m.assert_async().await;
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", super::TV_PATH)
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
            .list_tvshow_changes(&Default::default())
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
            .mock("GET", super::TV_PATH)
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
            .list_tvshow_changes(&Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);

        m.assert_async().await;
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::ChangeList;
    use crate::client::Client;
    use crate::client::reqwest::ReqwestExecutor;
    use crate::prelude::Command;

    #[tokio::test]
    async fn execute_tv() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);

        let result = client
            .list_tvshow_changes(&Default::default())
            .await
            .unwrap();
        assert_eq!(result.page, 1);
    }

    #[tokio::test]
    async fn execute_movie() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);

        let result = client
            .list_movie_changes(&Default::default())
            .await
            .unwrap();
        assert_eq!(result.page, 1);
    }

    #[tokio::test]
    async fn execute_person() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);

        let result = client
            .list_person_changes(&Default::default())
            .await
            .unwrap();
        assert_eq!(result.page, 1);
    }
}
