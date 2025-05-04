//! https://developer.themoviedb.org/reference/tv-series-aggregate-credits

use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub struct TVShowAggregateCredits {
    pub id: u64,
    pub cast: Vec<CastPerson>,
    pub crew: Vec<CrewPerson>,
}

#[derive(Debug, Deserialize)]
pub struct CastPerson {
    #[serde(flatten)]
    pub inner: Person,
    pub roles: Vec<Role>,
    pub order: u64,
}

#[derive(Debug, Deserialize)]
pub struct CrewPerson {
    #[serde(flatten)]
    pub inner: Person,
    pub jobs: Vec<Job>,
    pub department: String,
}

#[derive(Debug, Deserialize)]
pub struct Person {
    pub id: u64,
    pub adult: bool,
    pub gender: u64,
    pub known_for_department: String,
    pub name: String,
    pub original_name: String,
    pub popularity: f64,
    pub profile_path: Option<String>,
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

#[derive(Clone, Debug, Default, Serialize)]
pub struct GetTVShowAggregateCreditsParams<'a> {
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<Cow<'a, str>>,
}

impl<'a> GetTVShowAggregateCreditsParams<'a> {
    pub fn set_language(&mut self, value: impl Into<Cow<'a, str>>) {
        self.language = Some(value.into());
    }

    pub fn with_language(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.set_language(value);
        self
    }
}

impl<E: crate::client::Executor> crate::Client<E> {
    /// Get tvshow aggregate credits
    ///
    /// ```rust
    /// use tmdb_api::client::Client;
    /// use tmdb_api::client::reqwest::ReqwestExecutor;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
    ///     match client.get_tvshow_aggregate_credits(42, &Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn get_tvshow_aggregate_credits(
        &self,
        tvshow_id: u64,
        params: &GetTVShowAggregateCreditsParams<'_>,
    ) -> crate::Result<TVShowAggregateCredits> {
        let url = format!("/tv/{tvshow_id}/aggregate_credits");
        self.execute(&url, params).await
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::Client;
    use crate::client::reqwest::ReqwestExecutor;

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

        let result = client
            .get_tvshow_aggregate_credits(1399, &Default::default())
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

        let err = client
            .get_tvshow_aggregate_credits(1399, &Default::default())
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

        let err = client
            .get_tvshow_aggregate_credits(1399, &Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::Client;
    use crate::client::reqwest::ReqwestExecutor;
    use crate::prelude::Command;

    use super::TVShowAggregateCredits;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);

        let result = client
            .get_tvshow_aggregate_credits(1399, &Default::default())
            .await
            .unwrap();
        assert_eq!(result.id, 1399);
        assert!(!result.cast.is_empty());
        assert!(!result.crew.is_empty());
    }
}
