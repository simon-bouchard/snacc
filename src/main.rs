fn main() {
    match snacc_lib::run() {
        Ok(msg) => {
            if !msg.is_empty() {
                println!("{}", msg);
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}