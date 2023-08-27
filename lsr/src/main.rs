fn main() {
    if let Err(e) = lsr::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
