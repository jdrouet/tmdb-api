use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerOtherBodyError {
    pub status_code: u16,
    pub status_message: String,
}

impl std::error::Error for ServerOtherBodyError {}
impl std::fmt::Display for ServerOtherBodyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "server body error with code {}: {}",
            self.status_code, self.status_message
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerValidationBodyError {
    pub errors: Vec<String>,
}

impl std::error::Error for ServerValidationBodyError {}
impl std::fmt::Display for ServerValidationBodyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "server validation body errors:")?;
        for item in self.errors.iter() {
            write!(f, ", {item}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Error)]
#[serde(untagged)]
pub enum ServerBodyError {
    #[error(transparent)]
    Other(#[from] ServerOtherBodyError),
    #[error(transparent)]
    Validation(#[from] ServerValidationBodyError),
}

impl ServerBodyError {
    pub fn as_other_error(&self) -> Option<&ServerOtherBodyError> {
        match self {
            Self::Other(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn as_validation_error(&self) -> Option<&ServerValidationBodyError> {
        match self {
            Self::Validation(inner) => Some(inner),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ServerError {
    pub code: u16,
    pub body: ServerBodyError,
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "server error with code {}: {}", self.code, self.body)
    }
}

impl std::error::Error for ServerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.body)
    }
}

#[cfg(feature = "commands")]
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Server(#[from] ServerError),
}

#[cfg(feature = "commands")]
impl Error {
    pub fn as_reqwest_error(&self) -> Option<&reqwest::Error> {
        match self {
            Self::Reqwest(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn is_reqwest_error(&self) -> bool {
        matches!(self, Self::Reqwest(_))
    }

    pub fn as_server_error(&self) -> Option<&ServerError> {
        match self {
            Self::Server(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn is_server_error(&self) -> bool {
        matches!(self, Self::Server(_))
    }
}

#[cfg(feature = "commands")]
impl From<(reqwest::StatusCode, ServerBodyError)> for Error {
    fn from((code, body): (reqwest::StatusCode, ServerBodyError)) -> Self {
        Self::Server(ServerError {
            code: code.as_u16(),
            body,
        })
    }
}
