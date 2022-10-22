use std::borrow::Cow;

/// Command to get the details of a tvshow episode
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::tvshow::episode::details::TVShowEpisodeDetails;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = TVShowEpisodeDetails::new(1, 1, 1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct TVShowEpisodeDetails {
    /// ID of the TV Show
    pub tv_id: u64,
    /// Number of the season
    pub season_number: u64,
    /// Number of the episode
    pub episode_number: u64,
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
}

impl TVShowEpisodeDetails {
    pub fn new(tv_id: u64, season_number: u64, episode_number: u64) -> Self {
        Self {
            tv_id,
            season_number,
            episode_number,
            language: None,
        }
    }

    pub fn with_language(mut self, value: Option<String>) -> Self {
        self.language = value;
        self
    }
}

impl crate::prelude::Command for TVShowEpisodeDetails {
    type Output = crate::tvshow::Episode;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!(
            "/tv/{}/season/{}/episode/{}",
            self.tv_id, self.season_number, self.episode_number
        ))
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        if let Some(language) = self.language.as_ref() {
            vec![("language", Cow::Borrowed(language.as_str()))]
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TVShowEpisodeDetails;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn it_works() {
        let _m = mock("GET", "/tv/1399/season/1/episode/1")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../../assets/tvshow-episode-success.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let result = TVShowEpisodeDetails::new(1399, 1, 1)
            .execute(&client)
            .await
            .unwrap();
        assert_eq!(result.inner.id, 63056);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let _m = mock("GET", "/tv/1399/season/1/episode/1")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../../assets/invalid-api-key.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let err = TVShowEpisodeDetails::new(1399, 1, 1)
            .execute(&client)
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let _m = mock("GET", "/tv/1399/season/1/episode/1")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../../assets/resource-not-found.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let err = TVShowEpisodeDetails::new(1399, 1, 1)
            .execute(&client)
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::TVShowEpisodeDetails;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        for (tv_id, season_id) in [(1, 1130462u64)] {
            let result = TVShowEpisodeDetails::new(tv_id, 1, 1)
                .execute(&client)
                .await
                .unwrap();
            assert_eq!(result.inner.id, season_id);
        }
    }
}
