fn main() {
    if let Err(e) = fortuner::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
