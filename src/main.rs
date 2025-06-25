pub mod hash;
pub mod error;

use std::env;
use std::process;

use hash::HashType;

fn main() {
    let mut args = env::args();
    if args.len() <= 1 {
        eprintln!("expected 1 argument, received 0");
        process::exit(-1);
    }

    args.next();

    if let Some(first_arg) = args.next() {
		let hash_flag = match HashType::from_str(first_arg.as_str()) {
			Some(h) => h,
			None => {
				eprintln!("expected '--SHA256' or '--SHA512' flag");
				process::exit(-1);
			}
		};

		if let Some(second_arg) = args.next() {
			match hash::get_hash(hash_flag, second_arg) {
				Ok(h) => {
					println!("\n--{}--\n{}\n", &first_arg[2..], hex::encode(h));
				}
				Err(e) => {
					eprintln!("error while processing hash: {}", e.to_string());
					process::exit(-1);
				}
			}
		}
    };
}
