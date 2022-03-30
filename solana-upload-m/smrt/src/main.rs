mod lib;
use lib::{upload_file, process_instruction};

fn main() {
	upload_file();
	process_instruction();
	println!("Hello rust");
}