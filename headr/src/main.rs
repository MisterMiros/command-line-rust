fn main() {
    if let Err(error) = headr::run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
