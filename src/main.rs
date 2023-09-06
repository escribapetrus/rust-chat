mod server;

fn main() {
    if let Err(e) = server::init() {
	eprintln!("Error: {}", e);
    }
}
