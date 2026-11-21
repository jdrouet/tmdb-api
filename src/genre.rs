//! https://developer.themoviedb.org/reference/genre-movie-list
//! https://developer.themoviedb.org/reference/genre-tv-list

use crate::client::Executor;

const TV_PATH: &str = "/genre/tv/list";
const MOVIE_PATH: &str = "/genre/movie/list";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    pub genres: Vec<Genre>,
}

pub type Params<'a> = crate::common::LanguageParams<'a>;

impl<E: Executor> crate::Client<E> {
    /// List genres for movies
    ///
    /// ```rust
    /// use tmdb_api::Client;
    /// use tmdb_api::client::reqwest::Client as ReqwestClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestClient>::new("this-is-my-secret-token".into());
    ///     match client.list_movie_genres(&Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn list_movie_genres(&self, params: &Params<'_>) -> crate::Result<Response> {
        self.execute(MOVIE_PATH, params).await
    }

    /// List genres for tvshows
    ///
    /// ```rust
    /// use tmdb_api::Client;
    /// use tmdb_api::client::reqwest::Client as ReqwestClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestClient>::new("this-is-my-secret-token".into());
    ///     match client.list_tvshow_genres(&Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn list_tvshow_genres(&self, params: &Params<'_>) -> crate::Result<Response> {
        self.execute(TV_PATH, params).await
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::client::Client;
    use crate::client::reqwest::Client as ReqwestClient;

    #[tokio::test]
    async fn movie_works() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", super::MOVIE_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../assets/genre-movie-list.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let result = client.list_movie_genres(&Default::default()).await.unwrap();
        assert!(!result.genres.is_empty());

        m.assert_async().await;
    }

    #[tokio::test]
    async fn tv_works() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", super::TV_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../assets/genre-tv-list.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let result = client
            .list_tvshow_genres(&Default::default())
            .await
            .unwrap();
        assert!(!result.genres.is_empty());

        m.assert_async().await;
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", super::TV_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../assets/invalid-api-key.json"))
            .create_async()
            .await;
        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let err = client
            .list_tvshow_genres(&Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 7);

        m.assert_async().await;
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", super::TV_PATH)
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../assets/resource-not-found.json"))
            .create_async()
            .await;
        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let err = client
            .list_tvshow_genres(&Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
        m.assert_async().await;
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::Params;
    use crate::client::Client;
    use crate::client::reqwest::Client as ReqwestClient;

    #[tokio::test]
    async fn execute_tv() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestClient>::new(secret);
        let result = client
            .list_tvshow_genres(&Params::default().with_language("en-US"))
            .await
            .unwrap();
        assert!(!result.genres.is_empty());
    }

    #[tokio::test]
    async fn execute_movie() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestClient>::new(secret);
        let result = client
            .list_movie_genres(&Params::default().with_language("en-US"))
            .await
            .unwrap();
        assert!(!result.genres.is_empty());
    }
}
