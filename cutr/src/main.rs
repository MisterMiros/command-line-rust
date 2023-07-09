fn main() {
    if let Err(e) = cutr::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
