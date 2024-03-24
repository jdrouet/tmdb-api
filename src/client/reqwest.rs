use std::borrow::Cow;

#[derive(Debug, Default)]
pub struct ReqwestExecutor {
    inner: reqwest::Client,
}

impl From<reqwest::Client> for ReqwestExecutor {
    fn from(inner: reqwest::Client) -> Self {
        Self { inner }
    }
}

impl From<reqwest::Error> for crate::error::Error {
    fn from(value: reqwest::Error) -> Self {
        crate::error::Error::Request {
            source: Box::new(value),
        }
    }
}

#[async_trait::async_trait]
impl super::prelude::Executor for ReqwestExecutor {
    async fn execute<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        params: Vec<(&str, Cow<'_, str>)>,
    ) -> Result<T, crate::error::Error> {
        let res = self
            .inner
            .get(url)
            .query(&params)
            .send()
            .await
            .map_err(|err| crate::error::Error::Request {
                source: Box::new(err),
            })?;

        let status_code = res.status();
        if status_code.is_success() {
            res.json::<T>()
                .await
                .map_err(|err| crate::error::Error::Response {
                    source: Box::new(err),
                })
        } else if status_code == reqwest::StatusCode::UNPROCESSABLE_ENTITY {
            let payload: crate::error::ServerValidationBodyError =
                res.json()
                    .await
                    .map_err(|err| crate::error::Error::Response {
                        source: Box::new(err),
                    })?;
            Err(crate::error::Error::Validation(payload))
        } else {
            let content: crate::error::ServerOtherBodyError =
                res.json()
                    .await
                    .map_err(|err| crate::error::Error::Response {
                        source: Box::new(err),
                    })?;
            Err(crate::error::Error::Server {
                code: status_code.as_u16(),
                content,
            })
        }

        // todo!()
    }
}
