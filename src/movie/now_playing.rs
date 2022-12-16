use crate::common::PaginatedResult;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Get a list of movies in theatres. This is a release type query that looks for
/// all movies that have a release type of 2 or 3 within the specified date range.
///
/// You can optionally specify a region prameter which will narrow the search
/// to only look for theatrical release dates within the specified country.
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::movie::now_playing::MovieNowPlaying;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let result = MovieNowPlaying::default().execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct MovieNowPlaying {
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
    /// Specify which page to query.
    pub page: Option<u32>,
    /// Specify a ISO 3166-1 code to filter release dates. Must be uppercase.
    pub region: Option<String>,
}

impl MovieNowPlaying {
    pub fn with_language(mut self, value: Option<String>) -> Self {
        self.language = value;
        self
    }

    pub fn with_page(mut self, value: Option<u32>) -> Self {
        self.page = value;
        self
    }

    pub fn with_region(mut self, value: Option<String>) -> Self {
        self.region = value;
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DateRange {
    #[serde(deserialize_with = "crate::util::optional_date::deserialize")]
    pub maximum: Option<NaiveDate>,
    #[serde(deserialize_with = "crate::util::optional_date::deserialize")]
    pub minimum: Option<NaiveDate>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MovieNowPlayingResult {
    #[serde(flatten)]
    pub inner: PaginatedResult<super::MovieShort>,
    pub dates: DateRange,
}

impl crate::prelude::Command for MovieNowPlaying {
    type Output = MovieNowPlayingResult;

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed("/movie/now_playing")
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        let mut res = Vec::new();
        if let Some(ref language) = self.language {
            res.push(("language", Cow::Borrowed(language.as_str())))
        }
        if let Some(ref page) = self.page {
            res.push(("page", Cow::Owned(page.to_string())))
        }
        if let Some(ref region) = self.region {
            res.push(("region", Cow::Borrowed(region.as_str())))
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::MovieNowPlaying;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn it_works() {
        let _m = mock("GET", "/movie/now_playing")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-now-playing.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let result = MovieNowPlaying::default().execute(&client).await.unwrap();
        assert_eq!(result.inner.page, 1);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let _m = mock("GET", "/movie/now_playing")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let err = MovieNowPlaying::default()
            .execute(&client)
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let _m = mock("GET", "/movie/now_playing")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let err = MovieNowPlaying::default()
            .execute(&client)
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::MovieNowPlaying;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        let _result = MovieNowPlaying::default().execute(&client).await.unwrap();
    }
}
