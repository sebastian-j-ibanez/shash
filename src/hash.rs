use std::process;
use crate::error::Error;

use sha2::{Digest, Sha256, Sha512};

// Types of hashes this program can process.
pub enum HashType {
	SHA256,
	SHA512,
}

impl HashType {
	const SHA256_FLAG: &str = "-sha256";
	const SHA512_FLAG: &str = "-sha512";
	
	pub fn to_str(&self) -> &str {
		match self {
			Self::SHA256 => HashType::SHA256_FLAG,
			Self::SHA512 => HashType::SHA512_FLAG,
		}
	}

	pub fn from_str(flag: &str) -> Option<HashType> {
		match flag {
			HashType::SHA256_FLAG => Some(HashType::SHA256),
			HashType::SHA512_FLAG => Some(HashType::SHA512),
			_ => None
		}
	}
}

// Open file, read contents into Vector<u8>.
// Exit program if there is an Error opening the file.
fn file_to_bytes(file_name: String) -> Vec<u8> {
	match std::fs::read(file_name) {
        Ok(file) => file,
        Err(e) => {
            let message = format!("unable to open file: {}", e.to_string());
            eprintln!("{}", message);
            process::exit(-1);
        }
	}
}

// Return the hash of a file. 
pub fn get_hash(hash_type: HashType, file_name: String) -> Result<Vec<u8>, Error> {
    let file = file_to_bytes(file_name);

	match hash_type {
		HashType::SHA256 => {
			let mut hasher = Sha256::new();
			hasher.update(file);
			let hash: [u8; 32] = hasher.finalize().into();
			return Ok(hash.to_vec())
		},
		HashType::SHA512 => {
			let mut hasher = Sha512::new();
			hasher.update(file);
			let hash: [u8; 64] = hasher.finalize().into();
			return Ok(hash.to_vec())
		}
	}
}
