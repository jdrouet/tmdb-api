#[derive(Debug, Deserialize, Serialize)]
pub struct ServerOtherBodyError {
    pub status_code: u16,
    pub status_message: String,
}

impl std::fmt::Display for ServerOtherBodyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Server Error with status code {}: {}",
            self.status_code, self.status_message
        )
    }
}

impl std::error::Error for ServerOtherBodyError {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerValidationBodyError {
    pub errors: Vec<String>,
}

impl std::fmt::Display for ServerValidationBodyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, error) in self.errors.iter().enumerate() {
            let formatted_idx = format!("{: >2}. ", idx + 1);
            write!(f, "{}{}\n", formatted_idx, error)?;
        }
        Ok(())
    }
}

impl std::error::Error for ServerValidationBodyError {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ServerBodyError {
    Other(ServerOtherBodyError),
    Validation(ServerValidationBodyError),
}

impl std::fmt::Display for ServerBodyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other(other) => {
                write!(f, "{other}")
            }
            Self::Validation(validation) => {
                write!(f, "{validation}")
            }
        }
    }
}

impl std::error::Error for ServerBodyError {}

impl From<ServerOtherBodyError> for ServerBodyError {
    fn from(inner: ServerOtherBodyError) -> Self {
        Self::Other(inner)
    }
}

impl From<ServerValidationBodyError> for ServerBodyError {
    fn from(inner: ServerValidationBodyError) -> Self {
        Self::Validation(inner)
    }
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
        write!(f, "Server Error with code {}: {}", self.code, self.body)
    }
}

impl std::error::Error for ServerError {}

#[cfg(feature = "commands")]
#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Server(ServerError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Reqwest(reqwest) => {
                write!(f, "{reqwest}")
            }
            Error::Server(server) => {
                write!(f, "{server}")
            }
        }
    }
}

impl std::error::Error for Error {}

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
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
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
