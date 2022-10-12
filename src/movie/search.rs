use serde::Deserialize;
use std::borrow::Cow;

const PATH: &str = "/search/movie";

#[derive(Clone, Debug, Deserialize)]
pub struct Item {
    pub id: u64,
    pub title: String,
    pub original_title: String,
    pub original_language: String,
    pub overview: String,
    #[serde(with = "crate::util::date")]
    pub release_date: chrono::NaiveDate,
    pub genre_ids: Vec<u64>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub adult: bool,
    pub popularity: f64,
    pub vote_count: u64,
    pub vote_average: f64,
    pub video: bool,
}

/// Command to search for movies
#[derive(Clone, Debug, Default)]
pub struct MovieSearch {
    /// Text query to search.
    pub query: String,
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
    /// Which page to query.
    pub page: Option<u32>,
    /// Whether to inlcude adult (pornography) content in the results.
    pub include_adult: bool,
    /// ISO 3166-1 code to filter release region. Must be uppercase.
    pub region: Option<String>,
    pub year: Option<u16>,
    pub primary_release_year: Option<u16>,
}

impl MovieSearch {
    pub fn new(query: String) -> Self {
        Self {
            query,
            language: None,
            page: None,
            include_adult: false,
            region: None,
            year: None,
            primary_release_year: None,
        }
    }
}

impl crate::prelude::Command for MovieSearch {
    type Output = crate::prelude::PaginatedResult<Item>;

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed(PATH)
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        let mut res = vec![("query", Cow::Borrowed(self.query.as_str()))];

        if let Some(language) = self.language.as_ref() {
            res.push(("language", Cow::Borrowed(language.as_str())));
        }
        if let Some(page) = self.page {
            res.push(("page", Cow::Owned(page.to_string())));
        }
        if self.include_adult {
            res.push(("include_adult", Cow::Borrowed("true")));
        }
        if let Some(region) = self.region.as_ref() {
            res.push(("region", Cow::Borrowed(region.as_str())));
        }
        if let Some(year) = self.year {
            res.push(("year", Cow::Owned(year.to_string())));
        }
        if let Some(primary_release_year) = self.primary_release_year {
            res.push((
                "primary_release_year",
                Cow::Owned(primary_release_year.to_string()),
            ));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::MovieSearch;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn it_works() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = MovieSearch::new("Whatever".into());

        let _m = mock("GET", super::PATH)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("query".into(), "Whatever".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/search-movie-success.json"))
            .create();
        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
        assert_eq!(result.results.len(), 14);
        assert_eq!(result.total_pages, 1);
        assert_eq!(result.total_results, 14);
        let item = result.results.first().unwrap();
        assert_eq!(item.title, "The Avengers");
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = MovieSearch::new("Whatever".into());

        let _m = mock("GET", super::PATH)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("query".into(), "Whatever".into()),
            ]))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create();
        let err = cmd.execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let cmd = MovieSearch::new("Whatever".into());

        let _m = mock("GET", super::PATH)
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api_key".into(), "secret".into()),
                Matcher::UrlEncoded("query".into(), "Whatever".into()),
            ]))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create();
        let err = cmd.execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::MovieSearch;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn search_rrrrrrr() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let cmd = MovieSearch::new("Rrrrrrr".into());

        let result = cmd.execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
        assert_eq!(result.results.len(), 1);
        assert_eq!(result.total_pages, 1);
        assert_eq!(result.total_results, 1);
        let item = result.results.first().unwrap();
        assert_eq!(item.title, "RRRrrrr!!!");
    }
}
