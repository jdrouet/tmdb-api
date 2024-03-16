use std::borrow::Cow;

use crate::common::PaginatedResult;

/// Get the release date along with the certification for a movie.
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::movie::reviews::MovieReviews;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = MovieReviews::new(1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {res:#?}"),
///         Err(err) => eprintln!("error: {err:?}"),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct MovieReviews {
    /// ID of the movie.
    pub movie_id: u64,
}

impl MovieReviews {
    pub fn new(movie_id: u64) -> Self {
        Self { movie_id }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorDetails {
    pub name: String,
    pub username: String,
    pub avatar_path: Option<String>,
    pub rating: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieReview {
    pub id: String,
    pub author: String,
    pub author_details: AuthorDetails,
    pub content: String,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl crate::prelude::Command for MovieReviews {
    type Output = PaginatedResult<MovieReview>;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/movie/{}/reviews", self.movie_id))
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

    use super::MovieReviews;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/movie/550/reviews")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/movie-reviews.json"))
            .create_async()
            .await;

        let result = MovieReviews::new(550).execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
        assert!(!result.results.is_empty());
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
            .mock("GET", "/movie/550/reviews")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = MovieReviews::new(550).execute(&client).await.unwrap_err();
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
            .mock("GET", "/movie/550/reviews")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = MovieReviews::new(550).execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::prelude::Command;
    use crate::Client;

    use super::MovieReviews;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        let result = MovieReviews::new(550).execute(&client).await.unwrap();
        assert_eq!(result.page, 1);
    }
}
