use crate::common::MediaType;
use std::borrow::Cow;

/// Command to get the details of a collection
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::collection::details::CollectionDetails;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = CollectionDetails::new(1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct CollectionDetails {
    /// ID of the collection
    pub collection_id: u64,
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CollectionDetailsResult {
    #[serde(flatten)]
    pub inner: super::CollectionBase,
    pub parts: Vec<Media>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Media {
    pub id: u64,
    pub media_type: MediaType,
    pub title: String,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    #[serde(default)]
    pub genre_ids: Vec<u64>,
    #[serde(default)]
    pub popularity: f64,
    #[serde(default)]
    pub adult: bool,
    #[serde(default)]
    pub video: bool,
    #[serde(default)]
    pub vote_average: f64,
    #[serde(default)]
    pub vote_count: u64,
    #[serde(default, deserialize_with = "crate::util::empty_string::deserialize")]
    pub release_date: Option<chrono::NaiveDate>,
}

impl CollectionDetails {
    pub fn new(collection_id: u64) -> Self {
        Self {
            collection_id,
            language: None,
        }
    }

    pub fn with_language(mut self, value: Option<String>) -> Self {
        self.language = value;
        self
    }
}

impl crate::prelude::Command for CollectionDetails {
    type Output = CollectionDetailsResult;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/collection/{}", self.collection_id))
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
    use super::CollectionDetails;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::Matcher;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/collection/10")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/collection-details.json"))
            .create_async()
            .await;

        let result = CollectionDetails::new(10).execute(&client).await.unwrap();
        assert_eq!(result.inner.id, 10);
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
            .mock("GET", "/collection/0")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = CollectionDetails::new(0)
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
            .mock("GET", "/collection/0")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = CollectionDetails::new(0)
            .execute(&client)
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::CollectionDetails;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        for i in &[10, 1196769] {
            let result = CollectionDetails::new(*i).execute(&client).await.unwrap();
            assert_eq!(result.inner.id, *i);
        }
    }
}
