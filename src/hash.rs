use std::process;

use sha2::{Digest, Sha256};

pub struct Error {
    message: String,
}

impl Error {
    fn init(message: &str) -> Error {
        Error {
            message: message.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        self.message.clone()
    }
}

pub fn get_hash(file_name: String) -> Result<[u8; 32], Error> {
    let file = match std::fs::read(file_name) {
        Ok(file) => file,
        Err(e) => {
            let message = format!("unable to open file: {}", e.to_string());
            Error::init(&message);
            process::exit(-1);
        }
    };

    let mut hasher = Sha256::new();
    hasher.update(file);
    Ok(hasher.finalize().into())
}
