use std::borrow::Cow;

/// Command to get alternative titles for a movie
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::movie::alternative_titles::MovieAlternativeTitles;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = MovieAlternativeTitles::new(1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct MovieAlternativeTitles {
    /// ID of the Movie
    pub movie_id: u64,
    /// The country to filter the alternative titles
    pub country: Option<String>,
}

impl MovieAlternativeTitles {
    pub fn new(movie_id: u64) -> Self {
        Self {
            movie_id,
            country: None,
        }
    }

    pub fn with_country(mut self, value: Option<String>) -> Self {
        self.country = value;
        self
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieAlternativeTitle {
    pub iso_3166_1: String,
    pub title: String,
    #[serde(
        deserialize_with = "crate::util::empty_string::deserialize",
        rename = "type"
    )]
    pub kind: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieAlternativeTitlesResult {
    pub id: u64,
    pub titles: Vec<MovieAlternativeTitle>,
}

impl crate::prelude::Command for MovieAlternativeTitles {
    type Output = MovieAlternativeTitlesResult;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/movie/{}/alternative_titles", self.movie_id))
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

    use super::MovieAlternativeTitles;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/movie/3/alternative_titles")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-alternative-titles.json"))
            .create_async()
            .await;

        let result = MovieAlternativeTitles::new(3)
            .execute(&client)
            .await
            .unwrap();
        assert_eq!(result.id, 550);
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
            .mock("GET", "/movie/1/alternative_titles")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = MovieAlternativeTitles::new(1)
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
            .mock("GET", "/movie/1/alternative_titles")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = MovieAlternativeTitles::new(1)
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

    use super::MovieAlternativeTitles;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        let result = MovieAlternativeTitles::new(3)
            .execute(&client)
            .await
            .unwrap();
        assert_eq!(result.id, 3);
    }
}
