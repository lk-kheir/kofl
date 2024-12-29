use std::fmt::{self, write};
use std::io;
pub enum ErrorValidation {
    EmptyName,
    LongName,
    UnrespectedPasswordProtocol,
    StoragePathNotFound
}




impl fmt::Display for ErrorValidation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            ErrorValidation::LongName  => write!(f, "Long name expected no more than 60 caracters (example))"),
            ErrorValidation::EmptyName  => write!(f, "Provid a name to be associated with Your Password"),
            ErrorValidation::UnrespectedPasswordProtocol  => write!(f, "Password does not respect the the security protocol"),
            ErrorValidation::StoragePathNotFound => write!(f,"Storage Path not found"),
        }
        
    }
}
pub enum ErrorExecution {
    IoError(io::Error),
    Unknown,
}

impl From<io::Error> for ErrorExecution {
    fn from(err: io::Error) -> ErrorExecution {
        ErrorExecution::IoError(err)
    }
}

impl fmt::Display for ErrorExecution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorExecution::IoError(e) => write!(f, "I/O Error: {}", e),
            ErrorExecution::Unknown => write!(f, "Something unexpected happened during execution"),
        }
    }
}
