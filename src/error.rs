//! Top level error types

#[derive(Debug, thiserror::Error)]
#[error("an application error occurred")]
pub struct AppError;

/// A suggestion displayed to the user
pub struct Suggestion(pub &'static str);