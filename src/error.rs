use std::fmt;

#[derive(Debug)]
pub enum Error {
    Zbus(zbus::Error),
    Fdo(zbus::fdo::Error),
    Json(serde_json::Error),
    /// Data mismatch when fetching user input queue slots
    UserInputSlotMismatch,
    BackendNotReady,
    MissingUserCredentials,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Error::Zbus(_), Error::Zbus(_)) => true,
            (Error::Json(_), Error::Json(_)) => true,
            (Error::UserInputSlotMismatch, Error::UserInputSlotMismatch) => true,
            (Error::BackendNotReady, Error::BackendNotReady) => true,
            (Error::MissingUserCredentials, Error::MissingUserCredentials) => true,
            (_, _) => false,
        }
    }
}

impl From<zbus::Error> for Error {
    fn from(err: zbus::Error) -> Self {
        Self::Zbus(err)
    }
}

impl From<zbus::fdo::Error> for Error {
    fn from(err: zbus::fdo::Error) -> Self {
        Self::Fdo(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Zbus(err) => write!(f, "D-Bus Error: {:?}", err),
            Error::Fdo(err) => write!(f, "D-Bus Error: {:?}", err),
            Error::Json(err) => write!(f, "JSON Error: {}", err),
            Error::UserInputSlotMismatch => write!(f, "Mismatch in User Input Queue Slot"),
            Error::BackendNotReady => write!(f, "Backend VPN process is not ready"),
            Error::MissingUserCredentials => write!(f, "Missing user credentials"),
        }
    }
}
