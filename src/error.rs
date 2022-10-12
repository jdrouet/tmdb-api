use reqwest::StatusCode;

#[derive(Debug, serde::Deserialize)]
pub struct ServerBodyError {
    pub status_code: u16,
    pub status_message: String,
}

#[derive(Debug)]
pub struct ServerError {
    pub code: StatusCode,
    pub body: ServerBodyError,
}

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Server(ServerError),
}

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

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl From<(StatusCode, ServerBodyError)> for Error {
    fn from((code, body): (StatusCode, ServerBodyError)) -> Self {
        Self::Server(ServerError { code, body })
    }
}
