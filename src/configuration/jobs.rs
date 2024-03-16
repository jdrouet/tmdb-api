//! https://developer.themoviedb.org/reference/configuration-jobs

use std::borrow::Cow;

/// Get a list of all jobs
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::configuration::jobs::Jobs;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let result = Jobs::default().execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {res:#?}"),
///         Err(err) => eprintln!("error: {err:?}"),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct Jobs {}

#[derive(Debug, Deserialize)]
pub struct JobsResult {
    pub department: String,
    pub jobs: Vec<String>,
}

impl Jobs {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::prelude::Command for Jobs {
    type Output = Vec<JobsResult>;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned("/configuration/jobs".to_owned())
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::Client;
    use crate::prelude::Command;

    use super::Jobs;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let client = Client::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();

        let _m = server
            .mock("GET", "/configuration/jobs")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/configuration-jobs.json"))
            .create_async()
            .await;

        let result = Jobs::default().execute(&client).await.unwrap();
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

        let _m = server
            .mock("GET", "/configuration/jobs")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create_async()
            .await;

        let err = Jobs::default()
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
            .mock("GET", "/configuration/jobs")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create_async()
            .await;

        let err = Jobs::default()
            .execute(&client)
            .await
            .unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use crate::Client;
    use crate::prelude::Command;

    use super::Jobs;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        let result = Jobs::default().execute(&client).await.unwrap();
        assert!(!result.is_empty());
    }
}
