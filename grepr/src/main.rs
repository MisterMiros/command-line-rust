fn main() {
    if let Err(error) = grepr::run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
