#[derive(Debug)]
pub struct FeuError(String);

impl From<std::io::Error> for FeuError {
    fn from(err: std::io::Error) -> Self {
        FeuError(err.kind().to_string())
    }
}

impl From<std::env::VarError> for FeuError {
    fn from(err: std::env::VarError) -> Self {
        FeuError(err.to_string())
    }
}

impl From<miniserde::Error> for FeuError {
    fn from(_err: miniserde::Error) -> Self {
        FeuError("Cannot read config json.".to_string())
    }
}
