use std::borrow::Cow;

use crate::common::image::Image;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Params<'a> {
    /// specify a comma separated list of ISO-639-1 values to query, for
    /// example: en,null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_image_language: Option<Cow<'a, str>>,
    /// ISO 639-1 value to display translated data for the fields that support
    /// it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Cow<'a, str>>,
}

impl<'a> Params<'a> {
    pub fn set_include_image_language(&mut self, value: impl Into<Cow<'a, str>>) {
        self.include_image_language = Some(value.into());
    }

    pub fn with_include_image_language(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.set_include_image_language(value);
        self
    }

    pub fn set_language(&mut self, value: impl Into<Cow<'a, str>>) {
        self.language = Some(value.into());
    }

    pub fn with_language(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.set_language(value);
        self
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTVshowImagesResponse {
    pub id: u64,
    pub backdrops: Vec<Image>,
    pub posters: Vec<Image>,
    pub logos: Vec<Image>,
}

impl<E: crate::client::Executor> crate::Client<E> {
    /// Get tvshow images
    ///
    /// ```rust
    /// use tmdb_api::client::Client;
    /// use tmdb_api::client::reqwest::Client as ReqwestClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestClient>::new("this-is-my-secret-token".into());
    ///     match client.get_tvshow_images(42, &Default::default()).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn get_tvshow_images(
        &self,
        tvshow_id: u64,
        params: &Params<'_>,
    ) -> crate::Result<GetTVshowImagesResponse> {
        let url = format!("/tv/{tvshow_id}/images");
        self.execute(&url, params).await
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::client::Client;
    use crate::client::reqwest::Client as ReqwestClient;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/550/images")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/tv-images.json"))
            .create_async()
            .await;
        let result = client
            .get_tvshow_images(550, &Default::default())
            .await
            .unwrap();
        assert_eq!(result.id, 550);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/42/images")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;
        let err = client
            .get_tvshow_images(42, &Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::<ReqwestClient>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/tv/42/images")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;
        let err = client
            .get_tvshow_images(42, &Default::default())
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::client::Client;
    use crate::client::reqwest::Client as ReqwestClient;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestClient>::new(secret);

        let result = client
            .get_tvshow_images(550, &Default::default())
            .await
            .unwrap();
        assert_eq!(result.id, 550);
    }
}
