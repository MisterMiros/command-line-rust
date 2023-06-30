fn main() {
    if let Err(error) = uniqr::run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}