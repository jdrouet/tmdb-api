use std::borrow::Cow;

/// Command to get images of a company
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::company::images::CompanyImages;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = CompanyImages::new(1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {res:#?}"),
///         Err(err) => eprintln!("error: {err:?}"),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct CompanyImages {
    /// ID of the Company
    pub company_id: u64,
}

impl CompanyImages {
    pub fn new(company_id: u64) -> Self {
        Self { company_id }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompanyImage {
    pub aspect_ratio: f64,
    pub file_path: String,
    pub height: u64,
    pub width: u64,
    pub id: String,
    pub file_type: String,
    pub vote_average: f64,
    pub vote_count: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompanyImagesResult {
    pub id: u64,
    pub logos: Vec<CompanyImage>,
}

impl crate::prelude::Command for CompanyImages {
    type Output = CompanyImagesResult;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/company/{}/images", self.company_id))
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

    use super::CompanyImages;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/company/1/images")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/company-images.json"))
            .create_async()
            .await;

        let result = CompanyImages::new(1).execute(&client).await.unwrap();
        assert_eq!(result.id, 1);
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
            .mock("GET", "/company/1/images")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = CompanyImages::new(1).execute(&client).await.unwrap_err();
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
            .mock("GET", "/company/1/images")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = CompanyImages::new(1).execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::prelude::Command;
    use crate::Client;

    use super::CompanyImages;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        let result = CompanyImages::new(1).execute(&client).await.unwrap();
        assert_eq!(result.id, 1);
    }
}
