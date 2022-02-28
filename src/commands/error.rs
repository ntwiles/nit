pub struct Error(pub String);

impl std::convert::From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            _ => Self(String::from("Unknown error!")),
        }
    }
}
