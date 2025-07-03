fn main() {
    match snacc_lib::run() {
        Ok(msg) => println!("{}", msg),
        Err(e) => eprintln!("{}", e),
    }
}
