use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {}

impl AppError {
    pub fn exit_code(&self) -> i32 {
        match self {
            _ => 1,
        }
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
