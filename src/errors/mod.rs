use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppError {
    pub code: u16,
    pub message: String,
}

impl AppError{
    pub fn build(code: u16) -> AppError {
        let mes = match code {
            400 => "Bad Request".to_string(),
            401 => "Unauthorized access".to_string(),
            404 => "Not Found or resource does not exit".to_string(),
            500 => "Internal server error".to_string(),
            _ => "Something went wrong".to_string(),
        };
        AppError{code, message: mes}
    }
}

