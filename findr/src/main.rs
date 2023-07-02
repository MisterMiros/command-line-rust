fn main() {
    if let Err(e) = findr::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
