use std::fmt;

pub enum ErrorValidation {
    EmptyName,
    LongName,
    UnrespectedPasswordProtocol,
}




impl fmt::Display for ErrorValidation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            ErrorValidation::LongName  => write!(f, "Long name expected no more than 60 caracters (example))"),
            ErrorValidation::EmptyName  => write!(f, "Provid a name to be associated with Your Password"),
            ErrorValidation::UnrespectedPasswordProtocol  => write!(f, "Password does not respect the the security protocol"),
        }
        
    }
}
pub enum ErrorExecution {
    Unknown
}


impl fmt::Display for ErrorExecution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            ErrorExecution::Unknown  => write!(f, "Something unexpected happened during execution"),
        }
        
    }
}
