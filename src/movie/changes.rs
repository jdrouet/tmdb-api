use std::borrow::Cow;

use chrono::NaiveDate;

/// Command to get changes for a movie
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::movie::changes::MovieChanges;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = MovieChanges::new(1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {res:#?}"),
///         Err(err) => eprintln!("error: {err:?}"),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct MovieChanges {
    /// ID of the Movie
    pub movie_id: u64,
    /// Filter the results with a start date.
    pub start_date: Option<NaiveDate>,
    /// Filter the results with an end date.
    pub end_date: Option<NaiveDate>,
    /// The country to filter the alternative titles
    pub page: Option<u32>,
}

impl MovieChanges {
    pub fn new(movie_id: u64) -> Self {
        Self {
            movie_id,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieChange {
    pub key: String,
    pub items: Vec<MovieChangeItem>,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieChangesResult {
    pub changes: Vec<MovieChange>,
}

impl crate::prelude::Command for MovieChanges {
    type Output = MovieChangesResult;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/movie/{}/changes", self.movie_id))
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::prelude::Command;
    use crate::Client;

    use super::MovieChanges;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/movie/3/changes")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-single-changes.json"))
            .create_async()
            .await;

        let result = MovieChanges::new(3).execute(&client).await.unwrap();
        assert_eq!(result.changes.len(), 1);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/movie/1/changes")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = MovieChanges::new(1).execute(&client).await.unwrap_err();
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

        let _m = server
            .mock("GET", "/movie/1/changes")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = MovieChanges::new(1).execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::prelude::Command;
    use crate::Client;

    use super::MovieChanges;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        let result = MovieChanges::new(1)
            .with_start_date(Some(chrono::NaiveDate::from_ymd_opt(2015, 3, 14).unwrap()))
            .with_end_date(Some(chrono::NaiveDate::from_ymd_opt(2019, 3, 14).unwrap()))
            .execute(&client)
            .await
            .unwrap();
        assert!(result.changes.is_empty());
    }
}
