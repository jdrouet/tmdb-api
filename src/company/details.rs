use std::borrow::Cow;

/// Command to get details of a company
///
/// ```rust
/// use tmdb_api::prelude::Command;
/// use tmdb_api::Client;
/// use tmdb_api::company::details::CompanyDetails;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new("this-is-my-secret-token".into());
///     let cmd = CompanyDetails::new(1);
///     let result = cmd.execute(&client).await;
///     match result {
///         Ok(res) => println!("found: {:#?}", res),
///         Err(err) => eprintln!("error: {:?}", err),
///     };
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct CompanyDetails {
    /// ID of the Company
    pub company_id: u64,
}

impl CompanyDetails {
    pub fn new(company_id: u64) -> Self {
        Self { company_id }
    }
}

impl crate::prelude::Command for CompanyDetails {
    type Output = super::Company;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/company/{}", self.company_id))
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::CompanyDetails;
    use crate::prelude::Command;
    use crate::Client;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn it_works() {
        let _m = mock("GET", "/company/1")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/company-details-success.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let result = CompanyDetails::new(1).execute(&client).await.unwrap();
        assert_eq!(result.inner.id, 1);
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let _m = mock("GET", "/company/1")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/invalid-api-key.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let err = CompanyDetails::new(1).execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 7);
    }

    #[tokio::test]
    async fn resource_not_found() {
        let _m = mock("GET", "/company/1")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/resource-not-found.json"))
            .create();

        let client = Client::new("secret".into()).with_base_url(mockito::server_url());
        let err = CompanyDetails::new(1).execute(&client).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.body.as_other_error().unwrap().status_code, 34);
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::CompanyDetails;
    use crate::prelude::Command;
    use crate::Client;

    #[tokio::test]
    async fn execute() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::new(secret);

        let result = CompanyDetails::new(1).execute(&client).await.unwrap();
        assert_eq!(result.inner.id, 1);
    }
}
