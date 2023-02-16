#[derive(Debug)]
pub struct FeuError(pub String);

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

impl From<nix::errno::Errno> for FeuError {
    fn from(err: nix::errno::Errno) -> Self {
        FeuError(err.to_string())
    }
}
