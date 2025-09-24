#[derive(Debug)]
pub enum DBError {
    Io(std::io::Error),
    InvalidArgument(String),
    NotFound(String),
    Corruption(String),
    Other(String),
}

impl std::fmt::Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBError::Io(err) => write!(f, "I/O error: {}", err),
            DBError::InvalidArgument(msg) => write!(f, "Parsing error: {}", msg),
            DBError::NotFound(msg) => write!(f, "Item not found: {}", msg),
            DBError::Corruption(msg) => write!(f, "Item not found: {}", msg),
            DBError::Other(msg) => write!(f, "Item not found: {}", msg),
        }
    }
}

pub type DBResult<T> = Result<T, DBError>;
