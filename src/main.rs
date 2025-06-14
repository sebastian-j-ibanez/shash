pub mod hash;

use std::env;
use std::process;

fn main() {
    let mut args = env::args();
    if args.len() <= 1 {
        eprintln!("expected 1 argument, received 0");
        process::exit(-1);
    }

    args.next();

    if let Some(arg) = args.next() {
        match hash::get_hash(arg) {
            Ok(h) => {
                println!("\nSHA256\n{}\n", hex::encode(h));
            }
            Err(e) => {
                eprintln!("error while processing hash: {}", e.to_string());
                process::exit(-1);
            }
        }
    };
}
