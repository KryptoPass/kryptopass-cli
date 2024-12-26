use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid arguments provided")]
    InvalidArguments,
}

impl AppError {
    /// Asigna un código de error basado en el tipo de error
    pub fn exit_code(&self) -> i32 {
        match self {
            AppError::InvalidArguments => 1,
        }
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
