fn main() {
    if let Err(error) = tailr::run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
