//! https://developer.themoviedb.org/reference/genre-movie-list
//! https://developer.themoviedb.org/reference/genre-tv-list

use std::borrow::Cow;

use super::Genre;

const TV_PATH: &str = "/genre/tv/list";
const MOVIE_PATH: &str = "/genre/movie/list";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenreResult {
    pub genres: Vec<Genre>,
}

/// Command to list genres
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::genre::list::GenreList;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = GenreList::tv();
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {res:#?}"),
///         Err(err) => eprintln!("error: {err:?}"),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct GenreList {
    path: &'static str,
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
}

impl GenreList {
    pub fn tv() -> Self {
        Self {
            path: TV_PATH,
            language: None,
        }
    }

    pub fn movie() -> Self {
        Self {
            path: MOVIE_PATH,
            language: None,
        }
    }
}

#[async_trait::async_trait]
impl crate::prelude::Command for GenreList {
    type Output = Vec<Genre>;

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed(self.path)
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        if let Some(language) = self.language.as_ref() {
            vec![("language", Cow::Borrowed(language.as_str()))]
        } else {
            Vec::new()
        }
    }

    async fn execute(&self, client: &crate::Client) -> Result<Self::Output, crate::error::Error> {
        client
            .execute::<GenreResult>(self.path().as_ref(), self.params())
            .await
            .map(|res| res.genres)
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::prelude::Command;
    use crate::Client;

    use super::GenreList;

    #[tokio::test]
    async fn movie_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let cmd = GenreList::movie();

        let _m = server
            .mock("GET", super::MOVIE_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/genre-movie-list.json"))
            .create_async()
            .await;
        let result = cmd.execute(&client).await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn tv_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let cmd = GenreList::tv();

        let _m = server
            .mock("GET", super::TV_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/genre-tv-list.json"))
            .create_async()
            .await;
        let result = cmd.execute(&client).await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let cmd = GenreList::tv();

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
        let cmd = GenreList::tv();

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
    use crate::prelude::Command;
    use crate::Client;

    use super::GenreList;

    #[tokio::test]
    async fn execute_tv() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let mut cmd = GenreList::tv();
        cmd.language = Some("en-US".into());

        let result = cmd.execute(&client).await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn execute_movie() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);
        let mut cmd = GenreList::movie();
        cmd.language = Some("en-US".into());

        let result = cmd.execute(&client).await.unwrap();
        assert!(!result.is_empty());
    }
}
