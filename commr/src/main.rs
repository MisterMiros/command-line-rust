fn main() {
    if let Err(error) = commr::run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
