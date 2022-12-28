fn main() {
    if let Err(e) = echoer::get_args().and_then(echoer::run) {
        eprintln!("{}", e);
        std::process::exit(1)
    }
}
