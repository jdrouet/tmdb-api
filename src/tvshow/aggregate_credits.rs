//! https://developer.themoviedb.org/reference/tv-series-aggregate-credits

use std::borrow::Cow;

/// Command to get the aggregate credits of a TV show.
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::client::reqwest::ReqwestExecutor;
/// use tmdb_api::tvshow::aggregate_credits::TVShowAggregateCredits;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
///     let cmd = TVShowAggregateCredits::new(1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {res:#?}"),
///         Err(err) => eprintln!("error: {err:?}"),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct TVShowAggregateCredits {
    pub id: u64,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TVShowAggregateCreditsResult {
    pub id: u64,
    pub cast: Vec<CastPerson>,
    pub crew: Vec<CrewPerson>,
}

#[derive(Debug, Deserialize)]
pub struct CastPerson {
    pub id: u64,
    pub adult: bool,
    pub gender: u64,
    pub known_for_department: String,
    pub name: String,
    pub original_name: String,
    pub popularity: f64,
    pub profile_path: Option<String>,
    pub roles: Vec<Role>,
    pub total_episode_count: u64,
    pub order: u64,
}

#[derive(Debug, Deserialize)]
pub struct CrewPerson {
    pub id: u64,
    pub adult: bool,
    pub gender: u64,
    pub known_for_department: String,
    pub name: String,
    pub original_name: String,
    pub popularity: f64,
    pub profile_path: Option<String>,
    pub jobs: Vec<Job>,
    pub department: String,
    pub total_episode_count: u64,
}

#[derive(Debug, Deserialize)]
pub struct Role {
    pub credit_id: String,
    pub character: String,
    pub episode_count: u64,
}

#[derive(Debug, Deserialize)]
pub struct Job {
    pub credit_id: String,
    pub job: String,
    pub episode_count: u64,
}

impl TVShowAggregateCredits {
    pub fn new(tv_show_id: u64) -> Self {
        Self {
            id: tv_show_id,
            language: None,
        }
    }

    pub fn with_language(mut self, value: Option<String>) -> Self {
        self.language = value;
        self
    }
}

impl crate::prelude::Command for TVShowAggregateCredits {
    type Output = TVShowAggregateCreditsResult;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/tv/{}/aggregate_credits", self.id))
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        if let Some(ref language) = self.language {
            vec![("language", Cow::Borrowed(language))]
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::client::reqwest::ReqwestExecutor;
    use crate::prelude::Command;
    use crate::Client;

    use super::TVShowAggregateCredits;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/1399/aggregate_credits")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/tv-aggregate-credits.json"))
            .create_async()
            .await;

        let result = TVShowAggregateCredits::new(1399)
            .execute(&client)
            .await
            .unwrap();
        assert_eq!(result.id, 1399);
        assert!(!result.cast.is_empty());
        assert!(!result.crew.is_empty());
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
            .mock("GET", "/tv/1399/aggregate_credits")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = TVShowAggregateCredits::new(1399)
            .execute(&client)
            .await
            .unwrap_err();
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
            .mock("GET", "/tv/1399/aggregate_credits")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = TVShowAggregateCredits::new(1399)
            .execute(&client)
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::client::reqwest::ReqwestExecutor;
    use crate::prelude::Command;
    use crate::Client;

    use super::TVShowAggregateCredits;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);

        let result = TVShowAggregateCredits::new(1399)
            .execute(&client)
            .await
            .unwrap();
        assert_eq!(result.id, 1399);
        assert!(!result.cast.is_empty());
        assert!(!result.crew.is_empty());
    }
}
