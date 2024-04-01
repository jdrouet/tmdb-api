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

#[derive(Debug, Deserialize, Serialize, thiserror::Error)]
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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("couldn't execute request")]
    Request {
        #[source]
        source: Box<dyn std::error::Error + Send>,
    },
    #[error("couldn't read response")]
    Response {
        #[source]
        source: Box<dyn std::error::Error + Send>,
    },
    #[error(transparent)]
    Validation(ServerValidationBodyError),
    #[error("internal server error with code {code}")]
    Server {
        code: u16,
        #[source]
        content: ServerOtherBodyError,
    },
}

impl Error {
    pub fn as_validation_error(&self) -> Option<&ServerValidationBodyError> {
        match self {
            Self::Validation(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn as_server_error(&self) -> Option<&ServerOtherBodyError> {
        match self {
            Self::Server { code: _, content } => Some(content),
            _ => None,
        }
    }
}
