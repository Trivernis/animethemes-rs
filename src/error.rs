use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}
