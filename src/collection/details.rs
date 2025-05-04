use std::borrow::Cow;

use crate::{client::Executor, common::MediaType};

#[derive(Debug, Deserialize, Serialize)]
pub struct CollectionDetails {
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

#[derive(Clone, Debug, Default, Serialize)]
pub struct CollectionDetailsParams<'a> {
    /// ISO 639-1 value to display translated data for the fields that support it.
    pub language: Option<Cow<'a, str>>,
}

impl<'a> CollectionDetailsParams<'a> {
    pub fn set_language(&mut self, value: impl Into<Cow<'a, str>>) {
        self.language = Some(value.into());
    }

    pub fn with_language(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.set_language(value);
        self
    }
}

impl<E: Executor> crate::Client<E> {
    pub async fn get_collection_details(
        &self,
        collection_id: u64,
        params: &CollectionDetailsParams<'_>,
    ) -> crate::Result<CollectionDetails> {
        self.execute(&format!("/collection/{collection_id}"), params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::client::Client;
    use crate::client::reqwest::ReqwestExecutor;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/collection/10")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/collection-details.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let result = client
            .get_collection_details(10, &Default::default())
            .await
            .unwrap();

        assert_eq!(result.inner.id, 10);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/collection/0")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let err = client
            .get_collection_details(0, &Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/collection/0")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let err = client
            .get_collection_details(0, &Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::client::Client;
    use crate::client::reqwest::ReqwestExecutor;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);

        for i in [10, 1196769] {
            let result = client
                .get_collection_details(0, &Default::default())
                .await
                .unwrap();
            assert_eq!(result.inner.id, i);
        }
    }
}
