use std::borrow::Cow;

/// Command to get the details of a TV show season.
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::tvshow::season::details::TVShowSeasonDetails;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = TVShowSeasonDetails::new(1, 1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {res:#?}"),
///         Err(err) => eprintln!("error: {err:?}"),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct TVShowSeasonDetails {
    /// ID of the TV Show
    pub tv_id: u64,
    /// Number of the season
    pub season_number: u64,
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
}

impl TVShowSeasonDetails {
    pub fn new(tv_id: u64, season_number: u64) -> Self {
        Self {
            tv_id,
            season_number,
            language: None,
        }
    }

    pub fn with_language(mut self, value: Option<String>) -> Self {
        self.language = value;
        self
    }
}

impl crate::prelude::Command for TVShowSeasonDetails {
    type Output = crate::tvshow::Season;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/tv/{}/season/{}", self.tv_id, self.season_number))
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
    use mockito::Matcher;

    use crate::prelude::Command;
    use crate::Client;

    use super::TVShowSeasonDetails;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/1399/season/1")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../../assets/tv-season-details.json"))
            .create_async()
            .await;

        let result = TVShowSeasonDetails::new(1399, 1)
            .execute(&client)
            .await
            .unwrap();
        assert_eq!(result.inner.id, 3624);
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
            .mock("GET", "/tv/1399/season/1")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = TVShowSeasonDetails::new(1399, 1)
            .execute(&client)
            .await
            .unwrap_err();
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
            .mock("GET", "/tv/1399/season/1")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = TVShowSeasonDetails::new(1399, 1)
            .execute(&client)
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::prelude::Command;
    use crate::Client;

    use super::TVShowSeasonDetails;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        for (tv_id, (season_id, season_number)) in [(1, (2328126, 1)), (97888, (140901, 4))] {
            let result = TVShowSeasonDetails::new(tv_id, season_number)
                .execute(&client)
                .await
                .unwrap();
            assert_eq!(result.inner.id, season_id);
        }
    }
}
