pub struct Error {
    message: String,
}

impl Error {
    pub fn init(message: &str) -> Error {
        Error {
            message: message.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        self.message.clone()
    }
}
