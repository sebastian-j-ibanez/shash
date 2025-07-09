pub mod hash;
pub mod error;

use std::env;
use std::process;

use hash::HashType;

fn main() {
    let mut args = env::args();
    if args.len() <= 1 {
        print_usage();
        process::exit(-1);
    }

    args.next();

    if let Some(flag_arg) = args.next() {
		let hash_flag = match HashType::from_str(flag_arg.as_str()) {
			Some(h) => h,
			None => {
				eprintln!(
                    "invalid flag: {}",
                    flag_arg.as_str()
                );
                print_usage();
				process::exit(-1);
			}
		};

		if let Some(file_arg) = args.next() {
			match hash::get_hash(hash_flag, file_arg) {
				Ok(h) => {
                    println!("{}", hex::encode(h));
				}
				Err(e) => {
					eprintln!("error while processing hash: {}", e.to_string());
					process::exit(-1);
				}
			}
		}
    };
}

fn print_usage() {
    println!("Usage: shash.exe [flag] [file]\n");
    println!("Flags:");
    println!("    -sha256    Get SHA256 hash of file.");
    println!("    -sha512    Get SHA512 hash of file.");
}
