use crate::client::Executor;

impl<E: Executor> crate::Client<E> {
    /// Command to get details of a company
    ///
    /// ```rust
    /// use tmdb_api::client::Client;
    /// use tmdb_api::client::reqwest::ReqwestExecutor;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::<ReqwestExecutor>::new("this-is-my-secret-token".into());
    ///     match client.get_company_details(1).await {
    ///         Ok(res) => println!("found: {:#?}", res),
    ///         Err(err) => eprintln!("error: {:?}", err),
    ///     };
    /// }
    /// ```
    pub async fn get_company_details(&self, company_id: u64) -> crate::Result<super::Company> {
        let path = format!("/company/{company_id}");
        self.execute::<super::Company, _>(&path, &()).await
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;
    use crate::client::reqwest::ReqwestExecutor;
    use mockito::Matcher;

    #[tokio::test]
    async fn it_works() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", "/company/1")
            .match_query(Matcher::UrlEncoded("api_key".into(), "secret".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(include_str!("../../assets/company-details.json"))
            .create_async()
            .await;

        let client = Client::<ReqwestExecutor>::builder()
            .with_api_key("secret".into())
            .with_base_url(server.url())
            .build()
            .unwrap();
        let result = client.get_company_details(1).await.unwrap();
        assert_eq!(result.inner.id, 1);

        m.assert_async().await;
    }

    #[tokio::test]
    async fn invalid_api_key() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", "/company/1")
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
        let err = client.get_company_details(1).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 7);

        m.assert_async().await;
    }

    #[tokio::test]
    async fn resource_not_found() {
        let mut server = mockito::Server::new_async().await;
        let m = server
            .mock("GET", "/company/1")
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
        let err = client.get_company_details(1).await.unwrap_err();
        let server_err = err.as_server_error().unwrap();
        assert_eq!(server_err.status_code, 34);

        m.assert_async().await;
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

        let result = client.get_company_details(1).await.unwrap();
        assert_eq!(result.inner.id, 1);
    }
}
