fn main() {
    if let Err(error) = wcr::run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}