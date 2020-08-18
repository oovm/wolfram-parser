use crate::WolframError;

impl From<std::io::Error> for WolframError {
    fn from(value: std::io::Error) -> Self {
        WolframError::runtime_error(value)
    }
}
