use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Open,
    InProgress,
    Closed,
    Resolved,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => {
                write!(f, "OPEN")
            }
            Self::InProgress => {
                write!(f, "IN PROGRESS")
            }
            Self::Resolved => {
                write!(f, "RESOLVED")
            }
            Self::Closed => {
                write!(f, "CLOSED")
            }
        }
    }
}
